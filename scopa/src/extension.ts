import * as vscode from "vscode";
import { WasmContext, Memory } from '@vscode/wasm-component-model';

import { tergo } from "./tergo";

export async function activate(context: vscode.ExtensionContext) {
  console.log("tergo activated");

  const filename = vscode.Uri.joinPath(
    context.extensionUri,
    'scopa.wasm'
  );

  console.log(`Looking for the WASM under ${filename}`);
  const bits = await vscode.workspace.fs.readFile(filename);
  const module = await WebAssembly.compile(bits);
  const wasmContext: WasmContext.Default = new WasmContext.Default();
  const instance = await WebAssembly.instantiate(module, {});
  wasmContext.initialize(new Memory.Default(instance.exports));
  const api = tergo._.exports.bind(
    instance.exports as tergo._.Exports,
    wasmContext
  );

  vscode.languages.registerDocumentFormattingEditProvider("r", {
    provideDocumentFormattingEdits(
      document,
      options,
      token
    ): vscode.TextEdit[] {
      let documentText = document.getText();
      console.log(`Formatting the document:\n${documentText}`);
      return [
        vscode.TextEdit.replace(
          new vscode.Range(
            document.lineAt(0).range.start,
            document.lineAt(document.lineCount - 1).range.end
          ),
          api.format(documentText)
        ),
      ];
    },
  });
}

// This method is called when your extension is deactivated
export function deactivate() {
  console.log("tergo deactivated");
}
