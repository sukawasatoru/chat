'use strict';

module.exports = {
    'root': true,
    'parser': '@typescript-eslint/parser',
    'parserOptions': {
        'useJSXTextNode': true,
        'project': './tsconfig.json',
        'sourceType': 'module'
    },
    'plugins': [
        '@typescript-eslint',
        'react-hooks'
    ],
    'rules': {
        '@typescript-eslint/adjacent-overload-signatures': 'warn',
        '@typescript-eslint/camelcase': 'warn',
        '@typescript-eslint/class-name-casing': 'warn',
        '@typescript-eslint/member-delimiter-style': 'warn',
        '@typescript-eslint/member-ordering': 'warn',
        '@typescript-eslint/no-for-in-array': 'warn',
        'react-hooks/rules-of-hooks': 'error',
        'react-hooks/exhaustive-deps': 'warn',
    },
};
