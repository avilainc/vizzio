#!/usr/bin/env node
"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const commander_1 = require("commander");
const chalk_1 = __importDefault(require("chalk"));
const program = new commander_1.Command();
program
    .name('vizzio')
    .description('🚀 Vizzio Automation Platform CLI')
    .version('1.0.0');
// Comando: workflow
program
    .command('workflow')
    .description('Gerenciar workflows')
    .action(() => {
    console.log(chalk_1.default.cyan('📋 Workflows:'));
    console.log(chalk_1.default.gray('  vizzio workflow list     - Listar workflows'));
    console.log(chalk_1.default.gray('  vizzio workflow create   - Criar novo workflow'));
    console.log(chalk_1.default.gray('  vizzio workflow run      - Executar workflow'));
});
// Comando: email
program
    .command('email')
    .description('Gerenciar emails')
    .action(() => {
    console.log(chalk_1.default.cyan('📧 Emails:'));
    console.log(chalk_1.default.gray('  vizzio email templates  - Listar templates'));
    console.log(chalk_1.default.gray('  vizzio email send       - Enviar email'));
});
// Comando: finance
program
    .command('finance')
    .description('Ferramentas financeiras')
    .action(() => {
    console.log(chalk_1.default.cyan('💰 Finance:'));
    console.log(chalk_1.default.gray('  vizzio finance invoice  - Gerar fatura'));
    console.log(chalk_1.default.gray('  vizzio finance expense  - Registrar despesa'));
});
// Comando: shortcuts
program
    .command('shortcuts')
    .description('Gerenciar atalhos')
    .action(() => {
    console.log(chalk_1.default.cyan('⚡ Shortcuts:'));
    console.log(chalk_1.default.gray('  vizzio shortcuts list   - Listar atalhos'));
    console.log(chalk_1.default.gray('  vizzio shortcuts create - Criar atalho'));
});
program.parse();
//# sourceMappingURL=index.js.map