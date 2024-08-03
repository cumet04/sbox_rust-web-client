{
  Architectures: [
    'x86_64',
  ],
  EphemeralStorage: {
    Size: 512,
  },
  FunctionName: 'sbox_rust-web-client',
  Handler: 'hello.handler',
  LoggingConfig: {
    LogFormat: 'Text',
    LogGroup: '/aws/lambda/sbox_rust-web-client',
  },
  MemorySize: 128,
  // 初回デプロイするときは指定する or 適当に作ってimport
  // Role: 'arn:aws:iam::000000000000:role/service-role/hoge-role',
  Runtime: 'provided.al2023',
  SnapStart: {
    ApplyOn: 'None',
  },
  Timeout: 3,
  TracingConfig: {
    Mode: 'PassThrough',
  },
}
