import * as vscode from 'vscode';
import axios from 'axios';

let statusBarItem: vscode.StatusBarItem;
let isEnabled = true;

export function activate(context: vscode.ExtensionContext) {
    console.log('🚀 Avila Copilot ativado!');

    // Status bar
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = "$(rocket) Avila";
    statusBarItem.tooltip = "Avila Copilot - Ativo";
    statusBarItem.command = 'avilacopilot.toggle';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);

    // Registrar comandos
    context.subscriptions.push(
        vscode.commands.registerCommand('avilacopilot.toggle', () => {
            isEnabled = !isEnabled;
            statusBarItem.text = isEnabled ? "$(rocket) Avila" : "$(circle-slash) Avila";
            statusBarItem.tooltip = isEnabled ? "Avila Copilot - Ativo" : "Avila Copilot - Desativado";
            vscode.window.showInformationMessage(
                `Avila Copilot ${isEnabled ? 'ativado' : 'desativado'}`
            );
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avilacopilot.complete', async () => {
            await triggerCompletion();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avilacopilot.detectBugs', async () => {
            await detectBugs();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avilacopilot.generateDocs', async () => {
            await generateDocs();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avilacopilot.generateTests', async () => {
            await generateTests();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('avilacopilot.chat', async () => {
            await openChat();
        })
    );

    // Inline completion provider
    const completionProvider = vscode.languages.registerInlineCompletionItemProvider(
        { pattern: '**' },
        {
            async provideInlineCompletionItems(document, position, context, token) {
                if (!isEnabled) return { items: [] };

                const config = vscode.workspace.getConfiguration('avilacopilot');
                if (!config.get('autoComplete')) return { items: [] };

                const code = document.getText();
                const language = document.languageId;
                const offset = document.offsetAt(position);

                try {
                    const completion = await getCompletion(code, language, offset);
                    if (!completion) return { items: [] };

                    return {
                        items: [{
                            insertText: completion,
                            range: new vscode.Range(position, position)
                        }]
                    };
                } catch (error) {
                    console.error('Erro ao obter completion:', error);
                    return { items: [] };
                }
            }
        }
    );
    context.subscriptions.push(completionProvider);
}

async function getCompletion(code: string, language: string, cursor: number): Promise<string | null> {
    const config = vscode.workspace.getConfiguration('avilacopilot');
    const serverUrl = config.get<string>('serverPath');
    const apiKey = config.get<string>('apiKey');
    const model = config.get<string>('model') || 'dolphin-mistral';

    if (!serverUrl || !apiKey) {
        vscode.window.showWarningMessage('Configure Avila Copilot: Server Path e API Key necessários');
        return null;
    }

    try {
        const response = await axios.post<{ completion: string }>(
            `${serverUrl}/v1/code/completions`,
            {
                code,
                language,
                cursor_position: cursor,
                model,
                max_tokens: 500
            },
            {
                headers: { Authorization: `Bearer ${apiKey}` },
                timeout: 5000
            }
        );

        return response.data.completion;
    } catch (error: any) {
        console.error('Erro na API:', error.message);
        return null;
    }
}

async function triggerCompletion() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;

    const code = editor.document.getText();
    const language = editor.document.languageId;
    const position = editor.selection.active;
    const offset = editor.document.offsetAt(position);

    vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: "Gerando código...",
        cancellable: false
    }, async () => {
        const completion = await getCompletion(code, language, offset);
        if (completion) {
            await editor.edit(editBuilder => {
                editBuilder.insert(position, completion);
            });
        }
    });
}

async function detectBugs() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;

    const config = vscode.workspace.getConfiguration('avilacopilot');
    const serverUrl = config.get<string>('serverPath');
    const apiKey = config.get<string>('apiKey');

    if (!serverUrl || !apiKey) return;

    const code = editor.document.getText();
    const language = editor.document.languageId;

    vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: "Detectando bugs...",
        cancellable: false
    }, async () => {
        try {
            const response = await axios.post<{
                choices: Array<{ message: { content: string } }>
            }>(
                `${serverUrl}/v1/chat/completions`,
                {
                    model: 'dolphin-mistral',
                    messages: [{
                        role: 'user',
                        content: `Analise o código ${language} e liste bugs:\n\n\`\`\`${language}\n${code}\n\`\`\``
                    }]
                },
                { headers: { Authorization: `Bearer ${apiKey}` } }
            );

            const bugs = response.data.choices[0].message.content;
            const panel = vscode.window.createWebviewPanel(
                'avilaBugs',
                'Bugs Detectados',
                vscode.ViewColumn.Beside,
                {}
            );
            panel.webview.html = `<pre>${bugs}</pre>`;
        } catch (error) {
            vscode.window.showErrorMessage('Erro ao detectar bugs');
        }
    });
}

async function generateDocs() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;

    const config = vscode.workspace.getConfiguration('avilacopilot');
    const serverUrl = config.get<string>('serverPath');
    const apiKey = config.get<string>('apiKey');

    if (!serverUrl || !apiKey) return;

    const code = editor.document.getText();
    const language = editor.document.languageId;

    vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: "Gerando documentação...",
        cancellable: false
    }, async () => {
        try {
            const response = await axios.post<{
                choices: Array<{ message: { content: string } }>
            }>(
                `${serverUrl}/v1/chat/completions`,
                {
                    model: 'dolphin-mistral',
                    messages: [{
                        role: 'user',
                        content: `Gere documentação detalhada para:\n\n\`\`\`${language}\n${code}\n\`\`\``
                    }]
                },
                { headers: { Authorization: `Bearer ${apiKey}` } }
            );

            const docs = response.data.choices[0].message.content;
            const panel = vscode.window.createWebviewPanel(
                'avilaDocs',
                'Documentação',
                vscode.ViewColumn.Beside,
                {}
            );
            panel.webview.html = `<pre>${docs}</pre>`;
        } catch (error) {
            vscode.window.showErrorMessage('Erro ao gerar documentação');
        }
    });
}

async function generateTests() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;

    const config = vscode.workspace.getConfiguration('avilacopilot');
    const serverUrl = config.get<string>('serverPath');
    const apiKey = config.get<string>('apiKey');

    if (!serverUrl || !apiKey) return;

    const code = editor.document.getText();
    const language = editor.document.languageId;

    vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: "Gerando testes...",
        cancellable: false
    }, async () => {
        try {
            const response = await axios.post<{
                choices: Array<{ message: { content: string } }>
            }>(
                `${serverUrl}/v1/chat/completions`,
                {
                    model: 'dolphin-mistral',
                    messages: [{
                        role: 'user',
                        content: `Gere testes unitários para:\n\n\`\`\`${language}\n${code}\n\`\`\``
                    }]
                },
                { headers: { Authorization: `Bearer ${apiKey}` } }
            );

            const tests = response.data.choices[0].message.content;
            const doc = await vscode.workspace.openTextDocument({
                content: tests,
                language: language
            });
            await vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside);
        } catch (error) {
            vscode.window.showErrorMessage('Erro ao gerar testes');
        }
    });
}

async function openChat() {
    const panel = vscode.window.createWebviewPanel(
        'avilaChat',
        'Avila Chat',
        vscode.ViewColumn.Beside,
        { enableScripts: true }
    );

    panel.webview.html = getChatHtml();
}

function getChatHtml(): string {
    return `
    <!DOCTYPE html>
    <html>
    <head>
        <style>
            body { font-family: sans-serif; padding: 20px; }
            #chat { height: 400px; overflow-y: auto; border: 1px solid #ccc; padding: 10px; margin-bottom: 10px; }
            #input { width: 100%; padding: 10px; }
        </style>
    </head>
    <body>
        <div id="chat"></div>
        <input id="input" placeholder="Digite sua mensagem..." />
        <script>
            const vscode = acquireVsCodeApi();
            document.getElementById('input').addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    const msg = e.target.value;
                    document.getElementById('chat').innerHTML += '<p><strong>Você:</strong> ' + msg + '</p>';
                    e.target.value = '';
                    // TODO: Enviar para API
                }
            });
        </script>
    </body>
    </html>
    `;
}

export function deactivate() {
    console.log('Avila Copilot desativado');
}
