{
  Config: {
    AuthType: 'NONE',
    InvokeMode: 'BUFFERED',
  },
  Permissions: [
    {
      Principal: '*',
      StatementId: 'FunctionURLAllowPublicAccess',
    },
  ],
}
