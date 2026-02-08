use anyhow::Result;
use rune_core::config::types::Personality;
use rune_core::features::Feature;
use rune_core::protocol::AskForApproval;
use rune_core::protocol::EventMsg;
use rune_core::protocol::Op;
use rune_core::protocol::SandboxPolicy;
use rune_protocol::config_types::ReasoningSummary;
use rune_protocol::user_input::UserInput;
use core_test_support::responses::mount_sse_sequence;
use core_test_support::responses::sse_completed;
use core_test_support::responses::start_mock_server;
use core_test_support::skip_if_no_network;
use core_test_support::test_rune::test_rune;
use core_test_support::wait_for_event;
use pretty_assertions::assert_eq;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn model_change_appends_model_instructions_developer_message() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let resp_mock = mount_sse_sequence(
        &server,
        vec![sse_completed("resp-1"), sse_completed("resp-2")],
    )
    .await;

    let mut builder = test_rune().with_model("gpt-5.2-rune");
    let test = builder.build(&server).await?;
    let next_model = "gpt-5.1-rune-max";

    test.rune
        .submit(Op::UserTurn {
            items: vec![UserInput::Text {
                text: "hello".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            cwd: test.cwd_path().to_path_buf(),
            approval_policy: AskForApproval::Never,
            sandbox_policy: SandboxPolicy::ReadOnly,
            model: test.session_configured.model.clone(),
            effort: test.config.model_reasoning_effort,
            summary: ReasoningSummary::Auto,
            collaboration_mode: None,
            personality: None,
        })
        .await?;
    wait_for_event(&test.rune, |ev| matches!(ev, EventMsg::TurnComplete(_))).await;

    test.rune
        .submit(Op::OverrideTurnContext {
            cwd: None,
            approval_policy: None,
            sandbox_policy: None,
            windows_sandbox_level: None,
            model: Some(next_model.to_string()),
            effort: None,
            summary: None,
            collaboration_mode: None,
            personality: None,
        })
        .await?;

    test.rune
        .submit(Op::UserTurn {
            items: vec![UserInput::Text {
                text: "switch models".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            cwd: test.cwd_path().to_path_buf(),
            approval_policy: AskForApproval::Never,
            sandbox_policy: SandboxPolicy::ReadOnly,
            model: next_model.to_string(),
            effort: test.config.model_reasoning_effort,
            summary: ReasoningSummary::Auto,
            collaboration_mode: None,
            personality: None,
        })
        .await?;
    wait_for_event(&test.rune, |ev| matches!(ev, EventMsg::TurnComplete(_))).await;

    let requests = resp_mock.requests();
    assert_eq!(requests.len(), 2, "expected two model requests");

    let second_request = requests.last().expect("expected second request");
    let developer_texts = second_request.message_input_texts("developer");
    let model_switch_text = developer_texts
        .iter()
        .find(|text| text.contains("<model_switch>"))
        .expect("expected model switch message in developer input");
    assert!(
        model_switch_text.contains("The user was previously using a different model."),
        "expected model switch preamble, got: {model_switch_text:?}"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn model_and_personality_change_only_appends_model_instructions() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;
    let resp_mock = mount_sse_sequence(
        &server,
        vec![sse_completed("resp-1"), sse_completed("resp-2")],
    )
    .await;

    let mut builder = test_rune()
        .with_model("gpt-5.2-rune")
        .with_config(|config| {
            config.features.enable(Feature::Personality);
        });
    let test = builder.build(&server).await?;
    let next_model = "exp-rune-personality";

    test.rune
        .submit(Op::UserTurn {
            items: vec![UserInput::Text {
                text: "hello".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            cwd: test.cwd_path().to_path_buf(),
            approval_policy: AskForApproval::Never,
            sandbox_policy: SandboxPolicy::ReadOnly,
            model: test.session_configured.model.clone(),
            effort: test.config.model_reasoning_effort,
            summary: ReasoningSummary::Auto,
            collaboration_mode: None,
            personality: None,
        })
        .await?;
    wait_for_event(&test.rune, |ev| matches!(ev, EventMsg::TurnComplete(_))).await;

    test.rune
        .submit(Op::OverrideTurnContext {
            cwd: None,
            approval_policy: None,
            sandbox_policy: None,
            windows_sandbox_level: None,
            model: Some(next_model.to_string()),
            effort: None,
            summary: None,
            collaboration_mode: None,
            personality: Some(Personality::Pragmatic),
        })
        .await?;

    test.rune
        .submit(Op::UserTurn {
            items: vec![UserInput::Text {
                text: "switch model and personality".into(),
                text_elements: Vec::new(),
            }],
            final_output_json_schema: None,
            cwd: test.cwd_path().to_path_buf(),
            approval_policy: AskForApproval::Never,
            sandbox_policy: SandboxPolicy::ReadOnly,
            model: next_model.to_string(),
            effort: test.config.model_reasoning_effort,
            summary: ReasoningSummary::Auto,
            collaboration_mode: None,
            personality: None,
        })
        .await?;
    wait_for_event(&test.rune, |ev| matches!(ev, EventMsg::TurnComplete(_))).await;

    let requests = resp_mock.requests();
    assert_eq!(requests.len(), 2, "expected two model requests");

    let second_request = requests.last().expect("expected second request");
    let developer_texts = second_request.message_input_texts("developer");
    assert!(
        developer_texts
            .iter()
            .any(|text| text.contains("<model_switch>")),
        "expected model switch message when model changes"
    );
    assert!(
        !developer_texts
            .iter()
            .any(|text| text.contains("<personality_spec>")),
        "did not expect personality update message when model changed in same turn"
    );

    Ok(())
}
