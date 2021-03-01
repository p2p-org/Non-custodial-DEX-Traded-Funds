import { struct } from 'superstruct';

function jsonRpcResult(resultDescription: any) {
  const jsonRpcVersion = struct.literal('2.0');
  return struct.union([
    struct({
      jsonrpc: jsonRpcVersion,
      id: 'string',
      error: 'any',
    }),
    struct({
      jsonrpc: jsonRpcVersion,
      id: 'string',
      error: 'null?',
      result: resultDescription,
    }),
  ]);
}

const AccountInfoResult = struct({
  executable: 'boolean',
  owner: 'string',
  lamports: 'number',
  data: 'any',
  rentEpoch: 'number?',
});

const ProgramAccountInfoResult = struct({
  pubkey: 'string',
  account: AccountInfoResult,
});

/**
 * Expected JSON RPC response for the "getProgramAccounts" message
 */
export const GetProgramAccountsRpcResult = jsonRpcResult(
  struct.array([ProgramAccountInfoResult]),
);
