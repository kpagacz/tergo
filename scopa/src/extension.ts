// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from "vscode";

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  console.log("tergo activated");

  vscode.languages.registerDocumentFormattingEditProvider("r", {
    provideDocumentFormattingEdits(
      document,
      options,
      token
    ): vscode.TextEdit[] {
      let documentText = document.getText();
      console.log(`Formatting the document:\n${ documentText }`);
      return [
        vscode.TextEdit.replace(
          new vscode.Range(
            document.lineAt(0).range.start,
            document.lineAt(document.lineCount - 1).range.end
          ),
          "Formatted code\n"
        ),
      ];
    },
  });
}

// This method is called when your extension is deactivated
export function deactivate() { }
