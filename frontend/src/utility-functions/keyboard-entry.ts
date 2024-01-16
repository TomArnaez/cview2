// TODO: This function tries to find out what scan code the user pressed, regardless of keyboard layout.
export async function getLocalizedScanCode(e: KeyboardEvent): Promise<String> {
    const keyText = e.key;
    const scanCode = e.code;
    return "";
}