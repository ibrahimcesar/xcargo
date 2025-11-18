# xcargo Documentation

This directory contains the official documentation for xcargo, built with [Docusaurus](https://docusaurus.io/).

## Development

### Prerequisites

- Node.js 20.0 or above
- npm or yarn

### Local Development

```bash
cd docs
npm install
npm start
```

This command starts a local development server and opens up a browser window. Most changes are reflected live without having to restart the server.

### Build

```bash
npm run build
```

This command generates static content into the `build` directory and can be served using any static contents hosting service.

### Deployment

The documentation is automatically deployed to GitHub Pages when changes are pushed to the main branch.

Manual deployment:

```bash
npm run deploy
```

## Documentation Structure

```
docs/
├── intro.md                 # Introduction
├── installation.md          # Installation guide
├── quick-start.md          # Quick start guide
├── guides/                 # User guides
│   ├── basic-usage.md
│   ├── target-management.md
│   ├── cross-compilation.md
│   └── ci-cd-integration.md
├── architecture/           # Architecture docs
│   ├── overview.md
│   ├── target-detection.md
│   ├── toolchain-management.md
│   ├── build-strategy.md
│   └── container-runtime.md
├── design/                 # Design decisions
│   ├── philosophy.md
│   ├── native-first.md
│   ├── tier-system.md
│   ├── container-strategy.md
│   └── trade-offs.md
├── reference/              # Reference docs
│   ├── targets.md
│   ├── configuration.md
│   ├── cli-commands.md
│   └── environment-variables.md
└── api/                    # API documentation
    ├── overview.md
    ├── target.md
    ├── requirements.md
    └── errors.md
```

## Contributing

To add or update documentation:

1. Edit or create markdown files in the `docs/` directory
2. Update `sidebars.ts` if adding new pages
3. Test locally with `npm start`
4. Submit a pull request

## Writing Guidelines

- Use clear, concise language
- Include code examples where appropriate
- Use admonitions (:::info, :::tip, :::warning, :::danger) for important notes
- Link to related documentation
- Keep technical accuracy in mind

## Learn More

- [Docusaurus Documentation](https://docusaurus.io/)
- [Markdown Features](https://docusaurus.io/docs/markdown-features)
