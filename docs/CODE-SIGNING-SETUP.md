# Code Signing Setup Guide

This document explains how to set up Windows code signing for Canon CLI when you're ready to eliminate SmartScreen warnings.

## 🎯 Overview

Code signing requires a certificate from a trusted Certificate Authority (CA). The certificate proves your identity and ensures the binary hasn't been tampered with.

## 💳 Certificate Options

### Recommended Providers (2024)
| Provider | OV Certificate | EV Certificate | Setup Complexity |
|----------|---------------|----------------|------------------|
| **SSL.com** | $149/year | $299/year | ⭐⭐ Easy |
| **Sectigo** | $199/year | $399/year | ⭐⭐⭐ Medium |
| **DigiCert** | $299/year | $499/year | ⭐⭐⭐⭐ Advanced |

### Certificate Types
- **OV (Organization Validated)**: Shows company name, good for most projects
- **EV (Extended Validation)**: Highest trust, requires hardware token, immediate reputation

## 📋 Setup Process

### Step 1: Purchase Certificate
1. Choose a provider (SSL.com recommended for small projects)
2. Complete identity verification process
3. Receive certificate file (.p12 or .pfx format)

### Step 2: Test Certificate Locally
```cmd
# Install certificate in local store (Windows)
certlm.msc

# Test signing with signtool (Windows SDK)
signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com canon.exe
```

### Step 3: Prepare for GitHub Actions
```bash
# Convert certificate to base64 for GitHub Secrets
certutil -encode certificate.pfx certificate-base64.txt

# OR on Linux/Mac:
base64 -i certificate.pfx -o certificate-base64.txt
```

### Step 4: Add GitHub Secrets
Go to your repository → Settings → Secrets and variables → Actions

Add these secrets:
```
WINDOWS_CERTIFICATE     # Base64 encoded .pfx file (from step 3)
CERTIFICATE_PASSWORD    # Certificate password
CERTIFICATE_SHA1        # SHA1 thumbprint (optional, for specific cert selection)
```

### Step 5: Enable Signing
The code signing is already integrated in `.github/workflows/release.yml` but disabled by default. Once you add the secrets, it will automatically sign Windows binaries.

## 🔧 GitHub Secrets Setup

### Get Certificate Info
```cmd
# Get SHA1 thumbprint (Windows)
certutil -dump certificate.pfx
# Look for "Cert Hash(sha1):"
```

### Add Secrets via GitHub CLI
```bash
# If you have gh CLI installed
gh secret set WINDOWS_CERTIFICATE < certificate-base64.txt
gh secret set CERTIFICATE_PASSWORD --body "your_password_here"
gh secret set CERTIFICATE_SHA1 --body "your_sha1_thumbprint_here"
```

### Add Secrets via Web Interface
1. Go to `https://github.com/canon-protocol/canon-cli/settings/secrets/actions`
2. Click "New repository secret"
3. Add each secret listed above

## 🧪 Testing

### Test Locally (Windows)
```cmd
# Build the project
cargo build --release

# Sign manually (requires Windows SDK)
signtool sign /f certificate.pfx /p password /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 target/release/canon.exe

# Verify signature
signtool verify /pa target/release/canon.exe
```

### Test in GitHub Actions
1. Make a small change and commit
2. Create a test tag: `git tag v0.1.1-test`
3. Push: `git push origin v0.1.1-test`
4. Check Actions tab for build status
5. Download and test the signed binary

## 🎯 What Changes After Signing

### Before Signing
- Windows SmartScreen warning
- "Unknown publisher"
- Users must bypass manually

### After Signing  
- No SmartScreen warnings
- Shows your organization name
- Automatic trust by Windows

## 💡 Cost-Benefit Analysis

### Annual Costs
- **Certificate**: $150-300/year
- **Time investment**: ~4-8 hours initial setup
- **Maintenance**: ~1 hour/year (renewal)

### Benefits
- **Professional appearance**: No security warnings
- **User trust**: Official publisher identity
- **Reduced support**: Fewer "won't run" issues
- **Distribution options**: Can submit to Windows Store

## 🔄 Renewal Process

Certificates typically last 1-3 years:

1. **30 days before expiry**: Purchase renewal
2. **Get new certificate**: Download new .pfx file
3. **Update secrets**: Replace `WINDOWS_CERTIFICATE` and password
4. **Test**: Create test release to verify

## 🛡️ Security Best Practices

### Certificate Storage
- **Never commit** certificate files to git
- **Use strong passwords** for certificate
- **Store securely** on your local machine
- **Backup** certificate files safely

### GitHub Secrets
- **Minimal access**: Only for release workflows
- **Regular rotation**: Update passwords annually
- **Monitor usage**: Check Actions logs

## 📈 Implementation Timeline

### Immediate (Free)
- ✅ Document SmartScreen bypass for users
- ✅ Add security warnings to releases
- ✅ Provide checksums for verification

### Short-term (When budget allows)
1. Purchase OV certificate (~$150/year)
2. Set up local signing for testing
3. Configure GitHub Actions secrets
4. Test with pre-release
5. Enable for all releases

### Long-term
- Build reputation with Microsoft
- Consider EV certificate for maximum trust
- Explore Windows Store distribution

## 🚨 Current Status

- ✅ **Build pipeline ready** for code signing
- ✅ **Documentation complete** for users
- ⏳ **Certificate needed** to enable signing
- ⏳ **Secrets need setup** once certificate obtained

## 📞 Next Steps

1. **Evaluate budget**: $150-300/year for certificate
2. **Choose provider**: SSL.com recommended for start
3. **Purchase certificate**: Allow 1-3 days for validation
4. **Follow setup process** in this document
5. **Test thoroughly** before enabling for releases

---

**Remember**: Code signing is optional but highly recommended for professional Windows software distribution.