from __future__ import annotations

from enum import StrEnum
import logging
from typing import Any

from rune.cli.plan_offer.ports.whoami_gateway import WhoAmIGateway
from rune.core.config import ProviderConfig

logger = logging.getLogger(__name__)

CONSOLE_CLI_URL = "https://console.rune.ai/codestral/cli"
UPGRADE_URL = CONSOLE_CLI_URL
SWITCH_TO_PRO_KEY_URL = CONSOLE_CLI_URL


class PlanOfferAction(StrEnum):
    NONE = "none"
    UPGRADE = "upgrade"
    SWITCH_TO_PRO_KEY = "switch_to_pro_key"


class PlanType(StrEnum):
    FREE = "free"
    PRO = "pro"
    UNKNOWN = "unknown"


# Stubbed function - Rune is free and open source / local
async def decide_plan_offer(
    api_key: str | None, gateway: WhoAmIGateway
) -> tuple[PlanOfferAction, PlanType]:
    return PlanOfferAction.NONE, PlanType.PRO


def resolve_api_key_for_plan(provider: ProviderConfig) -> str | None:
    return None


def plan_offer_cta(action: PlanOfferAction) -> str | None:
    return None
