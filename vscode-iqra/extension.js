
const vscode = require('vscode');
const keywords = [
    'اذا', 'إذا', 'وإلا', 'والا', 'وإلاّ', 'بينما', 'صحيح', 'خطأ', 'و', 'أو', 'ليس', 'دالة', 'ارجع', 'جرب', 'امسك',
    'if', 'else', 'while', 'true', 'false', 'and', 'or', 'not', 'function', 'return', 'try', 'catch'
];

function activate(context) {
    // Completion
    context.subscriptions.push(
        vscode.languages.registerCompletionItemProvider('iqra', {
            provideCompletionItems(document, position) {
                return keywords.map(kw => new vscode.CompletionItem(kw, vscode.CompletionItemKind.Keyword));
            }
        })
    );

    // Error diagnostics: unmatched quotes, invalid keywords
    const diagnosticCollection = vscode.languages.createDiagnosticCollection('iqra');
    context.subscriptions.push(diagnosticCollection);

    function validateDocument(document) {
        if (document.languageId !== 'iqra') return;
        const diagnostics = [];
        const text = document.getText();
        // Unmatched quotes
        const quoteCount = (text.match(/"/g) || []).length;
        if (quoteCount % 2 !== 0) {
            diagnostics.push(new vscode.Diagnostic(
                new vscode.Range(0, 0, 0, 1),
                'علامة اقتباس غير مغلقة | Unmatched quote',
                vscode.DiagnosticSeverity.Error
            ));
        }
        // Invalid keywords (simple check)
        const wordRegex = /\b([\w\u0600-\u06FF]+)\b/g;
        let match;
        while ((match = wordRegex.exec(text)) !== null) {
            const word = match[1];
            if (/^[a-zA-Z\u0600-\u06FF]+$/.test(word) && !keywords.includes(word) && word.length > 2) {
                if (word === 'اطبع' || word === 'print') continue; // allow built-ins
                diagnostics.push(new vscode.Diagnostic(
                    new vscode.Range(document.positionAt(match.index), document.positionAt(match.index + word.length)),
                    `كلمة غير معروفة: ${word} | Unknown keyword: ${word}`,
                    vscode.DiagnosticSeverity.Warning
                ));
            }
        }
        diagnosticCollection.set(document.uri, diagnostics);
    }

    context.subscriptions.push(
        vscode.workspace.onDidOpenTextDocument(validateDocument)
    );
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(e => validateDocument(e.document))
    );
    context.subscriptions.push(
        vscode.workspace.onDidCloseTextDocument(doc => diagnosticCollection.delete(doc.uri))
    );

    // Direct execution: run .iqra file in terminal
    context.subscriptions.push(
        vscode.commands.registerCommand('iqra.runFile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showErrorMessage('لا يوجد ملف مفتوح | No file open');
                return;
            }
            const filePath = editor.document.fileName;
            if (!filePath.endsWith('.iqra')) {
                vscode.window.showErrorMessage('الملف ليس من نوع اقرأ | Not an Iqra file');
                return;
            }
            const terminal = vscode.window.createTerminal('Iqra Run');
            terminal.show();
            terminal.sendText(`cargo run -- "${filePath}"`);
            vscode.window.showInformationMessage('تم تنفيذ الملف | File executed');
        })
    );
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
