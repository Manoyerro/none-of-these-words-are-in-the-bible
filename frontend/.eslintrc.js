module.exports = {
  env: {
    browser: true,
    es2021: true,
    'jest/globals': true
  },
  extends: [
    'standard-with-typescript',
    'plugin:react/recommended'
  ],
  ignorePatterns: [
    'node_modules/**',
    'dist/**',
    'parcel-cache/**'
  ],
  overrides: [
    {
      env: {
        node: true
      },
      files: [
        '.eslintrc.{js,cjs}'
      ],
      parserOptions: {
        sourceType: 'script'
      }
    }
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    project: ['tsconfig.json'],
    sourceType: 'module'
  },
  plugins: [
    'react',
    'jest'
  ],
  rules: {
    semi: [1, 'never'],
    'sort-keys': 2
  }
}
