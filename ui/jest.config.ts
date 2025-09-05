// jest.config.ts
import type { Config } from 'jest';

const config: Config = {
  rootDir: process.cwd(),
  moduleFileExtensions: ['ts', 'tsx', 'js', 'jsx'],
  testEnvironment: 'jsdom',
  modulePathIgnorePatterns: ['spec'],
  transformIgnorePatterns: [],
  transform: {
    '.*\\.(tsx?|jsx?)$': [
      '@swc/jest',
      {
        jsc: {
          transform: { react: { runtime: 'automatic' } },
        },
      },
    ],
  },
  moduleNameMapper: {
    // ✅ correct: no leading/trailing slashes, backslashes escaped
    '\\.module\\.(css|scss|sass)$': 'identity-obj-proxy',
    '\\.(css|scss|sass)$': '<rootDir>/__mocks__/styleMock.js',
  },
};

export default config;
