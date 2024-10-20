import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  // By default, Docusaurus generates a sidebar from the docs folder structure
  // tutorialSidebar: [{type: 'autogenerated', dirName: '.'}],

  // But you can create a sidebar manually
  tutorialSidebar: [
      'intro',
      {
        type: 'category',
        label: 'Models',
        link: {
          type: 'doc',
          id: 'models/models',
        },
        items: [
            'models/Users/Users',
            {
                type: 'category',
                label: 'Travel',
                items: [
                    'models/Travel/Travel',
                    'models/Travel/Todo/Todo'
                ]
            }
        ],
      },
      {
        type: 'category',
        label: 'Data Structure',
        items: ['data-structure/schema', 'data-structure/data-structure'],
      }
  ],
};

export default sidebars;
