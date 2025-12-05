import express, { Router, Request, Response } from 'express';
import { Case } from '../models/Case';
import { CopilotAnalysisService } from '../services/CopilotAnalysisService';
import { EmailService } from '../services/EmailService';
import { ProposalGeneratorService } from '../services/ProposalGeneratorService';

const router = Router();
const analysisService = new CopilotAnalysisService(process.env.OPENAI_API_KEY!);
const proposalService = new ProposalGeneratorService();
const emailService = new EmailService(
  process.env.EMAIL_HOST!,
  parseInt(process.env.EMAIL_PORT!),
  process.env.EMAIL_USER!,
  process.env.EMAIL_PASSWORD!
);

// Criar novo caso
router.post('/cases', async (req: Request, res: Response) => {
  try {
    const {
      clientName,
      clientEmail,
      clientPhone,
      clientCompany,
      caseDescription,
      caseCategory,
      objectives,
      challenges,
      budget,
      timeline,
    } = req.body;

    const newCase = new Case({
      clientName,
      clientEmail,
      clientPhone,
      clientCompany,
      caseDescription,
      caseCategory,
      objectives,
      challenges,
      budget,
      timeline,
      status: 'draft',
    });

    await newCase.save();

    res.status(201).json({
      success: true,
      message: 'Caso criado com sucesso',
      caseId: newCase._id,
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      message: 'Erro ao criar caso',
      error,
    });
  }
});

// Obter caso por ID
router.get('/cases/:id', async (req: Request, res: Response) => {
  try {
    const caseData = await Case.findById(req.params.id);

    if (!caseData) {
      return res.status(404).json({
        success: false,
        message: 'Caso não encontrado',
      });
    }

    res.json({
      success: true,
      data: caseData,
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      message: 'Erro ao obter caso',
      error,
    });
  }
});

// Analisar caso com Copilot/IA
router.post('/cases/:id/analyze', async (req: Request, res: Response) => {
  try {
    const caseData = await Case.findById(req.params.id);

    if (!caseData) {
      return res.status(404).json({
        success: false,
        message: 'Caso não encontrado',
      });
    }

    // Atualizar status para "analyzing"
    caseData.status = 'analyzing';
    await caseData.save();

    // Fazer análise com Copilot
    const analysis = await analysisService.analyzeCase({
      caseDescription: caseData.caseDescription,
      clientCompany: caseData.clientCompany,
      objectives: caseData.objectives,
      challenges: caseData.challenges,
    });

    // Atualizar caso com análise
    caseData.analysis = analysis;
    caseData.status = 'analyzed';
    await caseData.save();

    res.json({
      success: true,
      message: 'Caso analisado com sucesso',
      analysis,
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      message: 'Erro ao analisar caso',
      error,
    });
  }
});

// Gerar e enviar proposta por email
router.post('/cases/:id/send-proposal', async (req: Request, res: Response) => {
  try {
    const caseData = await Case.findById(req.params.id);

    if (!caseData) {
      return res.status(404).json({
        success: false,
        message: 'Caso não encontrado',
      });
    }

    if (!caseData.analysis) {
      return res.status(400).json({
        success: false,
        message: 'Caso não foi analisado ainda',
      });
    }

    // Gerar HTML da proposta
    const proposalHTML = proposalService.generateHTML({
      clientName: caseData.clientName,
      clientCompany: caseData.clientCompany,
      strategy: caseData.analysis.strategy,
      recommendations: caseData.analysis.recommendations,
      timeline: caseData.analysis.timeline,
      estimatedBudget: caseData.analysis.estimatedBudget || 0,
      risks: caseData.analysis.risks,
      caseDescription: caseData.caseDescription,
    });

    // Enviar email
    await emailService.sendProposal({
      to: caseData.clientEmail,
      subject: `Proposta de Estratégia para ${caseData.clientName}`,
      html: proposalHTML,
      clientName: caseData.clientName,
    });

    // Atualizar caso
    caseData.proposal = {
      htmlContent: proposalHTML,
      sent: true,
      sentAt: new Date(),
    };
    caseData.status = 'proposal_sent';
    await caseData.save();

    res.json({
      success: true,
      message: 'Proposta enviada com sucesso',
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      message: 'Erro ao enviar proposta',
      error,
    });
  }
});

// Listar todos os casos
router.get('/cases', async (req: Request, res: Response) => {
  try {
    const page = parseInt(req.query.page as string) || 1;
    const limit = parseInt(req.query.limit as string) || 10;
    const skip = (page - 1) * limit;

    const cases = await Case.find()
      .skip(skip)
      .limit(limit)
      .sort({ createdAt: -1 });

    const total = await Case.countDocuments();

    res.json({
      success: true,
      data: cases,
      pagination: {
        page,
        limit,
        total,
        pages: Math.ceil(total / limit),
      },
    });
  } catch (error) {
    res.status(500).json({
      success: false,
      message: 'Erro ao listar casos',
      error,
    });
  }
});

export default router;
