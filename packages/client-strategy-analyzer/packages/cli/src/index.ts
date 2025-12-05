#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';

const program = new Command();

program
  .name('vizzio')
  .description('🚀 Vizzio Automation Platform CLI')
  .version('1.0.0');

// Comando: workflow
program
  .command('workflow')
  .description('Gerenciar workflows')
  .action(() => {
    console.log(chalk.cyan('📋 Workflows:'));
    console.log(chalk.gray('  vizzio workflow list     - Listar workflows'));
    console.log(chalk.gray('  vizzio workflow create   - Criar novo workflow'));
    console.log(chalk.gray('  vizzio workflow run      - Executar workflow'));
  });

// Comando: email
program
  .command('email')
  .description('Gerenciar emails')
  .action(() => {
    console.log(chalk.cyan('📧 Emails:'));
    console.log(chalk.gray('  vizzio email templates  - Listar templates'));
    console.log(chalk.gray('  vizzio email send       - Enviar email'));
  });

// Comando: finance
program
  .command('finance')
  .description('Ferramentas financeiras')
  .action(() => {
    console.log(chalk.cyan('💰 Finance:'));
    console.log(chalk.gray('  vizzio finance invoice  - Gerar fatura'));
    console.log(chalk.gray('  vizzio finance expense  - Registrar despesa'));
  });

// Comando: shortcuts
program
  .command('shortcuts')
  .description('Gerenciar atalhos')
  .action(() => {
    console.log(chalk.cyan('⚡ Shortcuts:'));
    console.log(chalk.gray('  vizzio shortcuts list   - Listar atalhos'));
    console.log(chalk.gray('  vizzio shortcuts create - Criar atalho'));
  });

program.parse();
