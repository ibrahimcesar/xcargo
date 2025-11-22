# Signing Setup for xcargo Releases

## Generate Minisign Key Pair

**One-time setup** (to be done by project maintainer):

```bash
# Generate key pair
minisign -G -p xcargo.pub -s xcargo.key

# You'll be prompted for a password - use a strong password
# This generates:
# - xcargo.pub  (public key - commit to repo)
# - xcargo.key  (private key - store as GitHub secret)
```

## Store Private Key in GitHub Secrets

1. Read the private key:
```bash
cat xcargo.key
```

2. Go to GitHub repository settings:
   - Settings → Secrets and variables → Actions
   - Click "New repository secret"
   - Name: `MINISIGN_KEY`
   - Value: (paste entire contents of xcargo.key)
   - Click "Add secret"

3. **IMPORTANT**: Also store the password:
   - Click "New repository secret"
   - Name: `MINISIGN_PASSWORD`
   - Value: (the password you used)
   - Click "Add secret"

## Commit Public Key

```bash
# Add public key to repository
git add xcargo.pub
git commit -m "feat: add minisign public key for release verification"
git push
```

## Update README

The public key will be displayed in README so users can verify downloads.

To get the public key string:
```bash
cat xcargo.pub
```

Copy the key (starts with `RW...`) and add it to README.

## Manual Signing (for testing)

To manually sign a binary:

```bash
# Sign a file
minisign -Sm target/release/xcargo -s xcargo.key

# This creates target/release/xcargo.minisig
```

To verify:
```bash
minisign -Vm target/release/xcargo -P $(cat xcargo.pub | grep -v 'minisign' | head -1)
```

## Security Best Practices

1. ✅ **Never commit xcargo.key** - it's in .gitignore
2. ✅ **Use strong password** for the key
3. ✅ **Store in GitHub Secrets** only
4. ✅ **Rotate key annually** - generate new key, update secret
5. ✅ **Document key rotation** in CHANGELOG

## Key Rotation Process

When rotating keys (annually or if compromised):

1. Generate new key pair
2. Update GitHub secret with new private key
3. Keep old public key in README (for old releases)
4. Add new public key to README
5. Announce in release notes
6. Document in CHANGELOG

## References

- [Minisign](https://jedisct1.github.io/minisign/)
- [GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
