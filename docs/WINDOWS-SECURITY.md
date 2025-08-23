# Windows Security

## SmartScreen Warning

When you download and run `canon-windows.exe`, you see:

```
Windows protected your PC
Microsoft Defender SmartScreen prevented an unrecognized app from starting.
Running this app might put your PC at risk.

App: canon-windows.exe
Publisher: Unknown publisher
```

**This is normal** for unsigned executables from open-source projects.

## For Users

### Method 1: "More Info" Bypass
1. Click **"More info"** in the SmartScreen dialog
2. Click **"Run anyway"** 
3. The app will run normally

### Method 2: Unblock File Properties
1. Right-click `canon-windows.exe`
2. Select **"Properties"**
3. Check **"Unblock"** at the bottom
4. Click **"OK"**
5. Run the exe normally

### Method 3: Command Line
```cmd
# Run once to bypass SmartScreen
powershell -Command "Unblock-File -Path 'canon-windows.exe'"

# Then run normally
canon-windows.exe --version
```

### Method 4: Verify File Integrity
```cmd
# Download the SHA256 checksum file
# Compare with: certutil -hashfile canon-windows.exe SHA256
# If they match, the file is authentic
```

## For Developers

### Option 1: Documentation (Free)
- Provide clear user instructions (above)
- Include checksums for verification
- Explain why the warning appears

### Option 2: Code Signing Certificate ($100-400/year)

#### Recommended Providers
| Provider | OV Certificate | EV Certificate | Notes |
|----------|---------------|----------------|-------|
| **SSL.com** | $150/year | $300/year | Good for small projects |
| **Sectigo** | $200/year | $400/year | Popular choice |
| **DigiCert** | $300/year | $500/year | Enterprise grade |
| **GlobalSign** | $250/year | $400/year | Well recognized |

#### Benefits of Code Signing
- No SmartScreen warnings
- Shows your company name as publisher
- Builds user trust
- Required for Windows Store distribution
- Prevents tampering detection

### Option 3: Alternative Distribution

#### Windows Package Manager
```cmd
# Future option - submit to winget
winget install canon-protocol.canon-cli
```

#### Chocolatey Package
```cmd
# Community package manager
choco install canon-cli
```

#### Installer Package
- Use NSIS or Inno Setup to create an installer
- Installers get fewer warnings than raw executables
- Can include additional setup steps

## Code Signing Implementation

### Step 1: Get Certificate
1. Purchase from a CA (Certificate Authority)
2. Verify your identity (for OV/EV certificates)
3. Download certificate files (.p12 or .pfx format)

### Step 2: GitHub Actions Integration
We can add code signing to the release workflow:

```yaml
# In .github/workflows/release.yml
- name: Sign Windows executable
  if: matrix.os == 'windows-latest'
  uses: dlemstra/code-sign-action@v1
  with:
    certificate: ${{ secrets.WINDOWS_CERTIFICATE }}
    password: ${{ secrets.CERTIFICATE_PASSWORD }}
    folder: 'target/${{ matrix.target }}/release/'
    recursive: false
```

### Step 3: Store Secrets
```bash
# Add to GitHub repository secrets:
# WINDOWS_CERTIFICATE (base64 encoded .pfx file)
# CERTIFICATE_PASSWORD (certificate password)
```

### Step 4: Updated Build Process
- Certificate stored securely in GitHub Secrets
- Signing happens automatically on release
- Signed binaries uploaded to GitHub Releases



## Security Best Practices

### For Users
1. Always download from official GitHub releases
2. Verify checksums when provided
3. Check the URL is correct: `github.com/canon-protocol/canon-cli`

### For Developers
1. Provide checksums for all releases
2. Use HTTPS for all distribution


## Notes

- SmartScreen warnings are normal for unsigned open-source software
- The binary is safe - it's built automatically from source code
- Users can safely bypass the warning using methods above
- Code signing is the only way to eliminate the warning entirely