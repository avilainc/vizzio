═══════════════════════════════════════════════════════
  🏗️ VIZZIO VIEWER v0.1.0 - Visualizador IFC 3D
═══════════════════════════════════════════════════════

📅 Data de Release: 09/12/2025

COMO USAR:
----------
1. Execute: vizzio-viewer.exe
2. Abra navegador: http://localhost:8080
3. Faça upload de um arquivo IFC ou use o exemplo incluído
4. Navegue com mouse:
   - Arrastar = Orbitar câmera
   - Scroll = Zoom

REQUISITOS MÍNIMOS:
-------------------
- Windows 10/11 (64-bit)
- 4GB RAM
- GPU com suporte WebGL 2.0
- Navegador moderno:
  ✓ Chrome 90+
  ✓ Edge 90+
  ✓ Firefox 88+

ARQUIVOS INCLUÍDOS:
-------------------
📦 vizzio-viewer.exe (0,31 MB)
   Servidor HTTP + Parser IFC nativo

🌐 static/ (Interface Web + WebAssembly)
   - index.html (Interface do usuário)
   - vizzio_viewer_bg.wasm (Motor de renderização)
   - vizzio_viewer.js (Bindings JavaScript)

🏗️ ELE - VZZ086_25 - Magnussão - Res. Heitor - REV01-4.ifc
   Projeto de exemplo para teste

TECNOLOGIA:
-----------
- 100% Rust nativo 🦀
- Zero dependências externas
- WebAssembly para performance
- WebGL 2.0 para renderização 3D
- Avila Stack proprietária (107 crates)

CARACTERÍSTICAS:
----------------
✨ Parser IFC STEP (ISO-10303-21)
⚡ Performance otimizada (60+ FPS)
💾 Sistema de cache inteligente
🎮 Controles intuitivos
📊 Métricas em tempo real

ROADMAP v0.2.0:
---------------
- Extração completa de geometrias
- Sistema de materiais e cores
- Seleção e highlight de objetos
- Ferramentas de medição
- Clipping planes
- Export glTF/OBJ

TROUBLESHOOTING:
----------------
❓ Servidor não inicia?
   → Verifique se porta 8080 está livre
   → Execute como administrador se necessário

❓ Página não carrega?
   → Certifique-se que está acessando http://localhost:8080
   → Verifique se pasta static/ está presente

❓ IFC não renderiza?
   → Arquivo pode estar corrompido
   → Tente o arquivo de exemplo incluído

SUPORTE:
--------
🐛 Issues: https://github.com/[seu-usuario]/vizzio/issues
📧 Email: [seu-email]
📚 Docs: https://github.com/[seu-usuario]/vizzio

LICENÇA:
--------
[Sua licença aqui]

═══════════════════════════════════════════════════════
Desenvolvido com ❤️ em Rust
© 2025 Vizzio Project
═══════════════════════════════════════════════════════
