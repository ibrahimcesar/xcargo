import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  tutorialSidebar: [
    'intro',
    'installation',
    'quick-start',
    {
      type: 'category',
      label: 'Guides',
      items: [
        'guides/basic-usage',
        'guides/target-management',
        'guides/cross-compilation',
        'guides/ci-cd-integration',
      ],
    },
    {
      type: 'category',
      label: 'Architecture',
      items: [
        'architecture/overview',
        'architecture/target-detection',
        'architecture/toolchain-management',
        'architecture/build-strategy',
        'architecture/container-runtime',
      ],
    },
    {
      type: 'category',
      label: 'Design Decisions',
      items: [
        'design/philosophy',
        'design/native-first',
        'design/tier-system',
        'design/container-strategy',
        'design/trade-offs',
      ],
    },
    {
      type: 'category',
      label: 'Reference',
      items: [
        'reference/targets',
        'reference/configuration',
        'reference/cli-commands',
        'reference/environment-variables',
      ],
    },
  ],

  apiSidebar: [
    {
      type: 'category',
      label: 'API Documentation',
      items: [
        'api/overview',
        'api/target',
        'api/requirements',
        'api/errors',
      ],
    },
  ],

  researchSidebar: [
    {
      type: 'category',
      label: 'Research & Internal Docs',
      collapsed: false,
      items: [
        'research/BUNDLED_TOOLCHAINS',
        'research/MACOS_TOOLCHAINS_RESEARCH',
        'research/WINDOWS_TOOLCHAINS_RESEARCH',
        'research/TOOLCHAIN_TESTING_FINDINGS',
        'research/BINARY_SIGNATURES',
        'research/SIGNING_SETUP',
      ],
    },
  ],
};

export default sidebars;
