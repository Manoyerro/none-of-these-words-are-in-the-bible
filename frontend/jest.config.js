module.exports = {
  setupFilesAfterEnv: [
    './setupTests.ts'
  ],
  testEnvironment: 'jsdom',
  transform: {
    '^.+\\.tsx?$': 'ts-jest'
  }
}
