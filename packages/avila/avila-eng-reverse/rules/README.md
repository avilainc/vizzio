# YARA Rules Directory

Place YARA rules here for malware detection:

## Structure

```
rules/
  malware/
    ransomware.yar
    trojans.yar
    rats.yar
  exploits/
    cve_rules.yar
  packers/
    upx.yar
    themida.yar
```

## Example Rule

```yara
rule Ransomware_Generic {
    meta:
        description = "Generic ransomware detection"
        author = "Deriax"
        date = "2025-12-05"

    strings:
        $enc1 = "encrypt" nocase
        $enc2 = "decrypt" nocase
        $ransom = "bitcoin" nocase
        $ext1 = ".locked"
        $ext2 = ".encrypted"

    condition:
        2 of ($enc*) and 1 of ($ransom, $ext*)
}
```

## Loading Rules

Rules are automatically loaded from this directory if configured in `config.toml`:

```toml
[malware]
yara_rules_path = "./rules/"
```
