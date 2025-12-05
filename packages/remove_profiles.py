import os
import re
from pathlib import Path

def remove_profile_sections(file_path):
    """Remove todas as seções [profile.*] de um arquivo Cargo.toml"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    original_content = content

    # Remove seções [profile.*] e todo o conteúdo até a próxima seção ou fim do arquivo
    # Padrão: [profile.xxx] seguido por qualquer coisa até a próxima [ ou fim
    pattern = r'\n\[profile\.[^\]]+\][^\[]*'
    content = re.sub(pattern, '', content)

    # Remove linhas vazias extras
    content = re.sub(r'\n\n\n+', '\n\n', content)

    if content != original_content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content.rstrip() + '\n')
        return True
    return False

def main():
    base_dirs = ['avila', 'avl', 'avx']
    updated_files = []

    for base_dir in base_dirs:
        if not os.path.exists(base_dir):
            continue

        for cargo_toml in Path(base_dir).rglob('Cargo.toml'):
            if remove_profile_sections(cargo_toml):
                updated_files.append(str(cargo_toml))
                print(f"✓ Updated: {cargo_toml}")

    print(f"\n✅ Total files updated: {len(updated_files)}")

if __name__ == '__main__':
    main()
