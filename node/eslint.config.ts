import js from '@eslint/js'
import * as typescriptParser from '@typescript-eslint/parser'
import typescript from '@typescript-eslint/eslint-plugin'
import globals from 'globals'

export default [
    // Base JavaScript recommended rules
    js.configs.recommended,

    // Configuration for TypeScript files
    {
        files: ['**/*.{ts,tsx,mts,cts}'],
        languageOptions: {
            parser: typescriptParser,
            parserOptions: {
                ecmaVersion: 'latest',
                sourceType: 'module',
            },
            globals: {
                // Browser globals
                window: 'readonly',
                document: 'readonly',
                console: 'readonly',
                // Add other browser globals as needed
            },
        },
        plugins: {
            '@typescript-eslint': typescript,
        },
        rules: {
            // TypeScript recommended rules
            ...typescript.configs.recommended.rules,

            // Add your custom rules here
            // Example:
            // '@typescript-eslint/no-unused-vars': 'error',
            // '@typescript-eslint/explicit-function-return-type': 'warn',
        },
    },

    // Configuration for test files
    {
        files: ['**/*.{test,spec}.{js,jsx,ts,tsx}', '**/tests/**/*.{js,jsx,ts,tsx}'],
        languageOptions: {
            parser: typescriptParser,
            parserOptions: {
                ecmaVersion: 'latest',
                sourceType: 'module',
            },
            globals: {
                ...globals.browser,
                ...globals.node,
                ...globals.jest,
            },
        },
        plugins: {
            '@typescript-eslint': typescript,
        },
        rules: {
            ...typescript.configs.recommended.rules,
        },
    },

    // Global ignores
    {
        ignores: [
            'node_modules/**',
            'dist/**',
            'build/**',
            '*.min.js',
        ],
    },
]
