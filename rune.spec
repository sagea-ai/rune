# -*- mode: python ; coding: utf-8 -*-

from PyInstaller.utils.hooks import collect_all

# Collect all dependencies (including hidden imports and binaries) from builtins modules
core_builtins_deps = collect_all('rune.core.tools.builtins')
# rune cli might use acp tools too? let's include them to be safe as they are in the same repo structure
acp_builtins_deps = collect_all('rune.acp.tools.builtins')

# Extract hidden imports and binaries, filtering to ensure only strings are in hiddenimports
hidden_imports = []
# Combined dependencies
for item in core_builtins_deps[2] + acp_builtins_deps[2]:
    if isinstance(item, str):
        hidden_imports.append(item)

binaries = core_builtins_deps[1] + acp_builtins_deps[1]

a = Analysis(
    ['rune/cli/entrypoint.py'],
    pathex=[],
    binaries=binaries,
    datas=[
        # By default, pyinstaller doesn't include the .md files
        ('rune/core/prompts/*.md', 'rune/core/prompts'),
        ('rune/core/tools/builtins/prompts/*.md', 'rune/core/tools/builtins/prompts'),
        # We also need to add all setup files
        ('rune/setup/*', 'rune/setup'),
        # Textual UI CSS file
        ('rune/cli/textual_ui/app.tcss', 'rune/cli/textual_ui'),
        # This is necessary because tools are dynamically called in rune, meaning there is no static reference to those files
        ('rune/core/tools/builtins/*.py', 'rune/core/tools/builtins'),
        ('rune/acp/tools/builtins/*.py', 'rune/acp/tools/builtins'),
    ],
    hiddenimports=hidden_imports,
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
    optimize=0,
)
pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name='rune',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
