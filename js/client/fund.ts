import BN from 'bn.js';
import * as borsh from 'borsh';
import { PublicKey, TransactionInstruction, Connection } from '@solana/web3.js';

class Assignable {
  constructor(properties) {
    Object.keys(properties).map((key) => {
      this[key] = properties[key];
    });
  }
}

class PubKey extends PublicKey {
  constructor({ value }) {
    super(value);
  }
}

class OptionAddress extends PublicKey {
  constructor({ value }) {
    super(value.slice(1, value.length));
  }
}

const PoolRequestTag = new BN('220a6cbdcd1cc4cf', 'hex');

export class PoolRequestInner extends Assignable {}

export class InitializeFundData extends Assignable {
  slippageDivider: BN;
  assetWeights: number[];
  initialSupply: BN;
}

export class InitializePoolRequest extends PoolRequestInner {
  nonce: number;
  assetsLength: number;
  poolName: string;
  feeRate: number;
  customData: Uint8Array;
}

export class PoolRequest extends Assignable {
  tag: BN;
  index: number;
  inner: PoolRequestInner;
}

export class AssetInfo extends Assignable {
  mint: PublicKey;
  vaultAddress: PublicKey;
}

export class ParamDesc extends Assignable {
  address: PublicKey;
  writable: number;
}

export class FundState extends Assignable {
  paused: number;
  slippageDivider: BN;
  assetWeights: number[];
  basicAsset: AssetInfo;
}

export class PoolState extends Assignable {
  tag: BN;
  poolTokenMint: PublicKey;
  assets: AssetInfo[];
  vaultSigner: PublicKey;
  vaultSignerNonce: number;
  accountParams: ParamDesc[];
  name: string;
  lqdFeeVault: PublicKey;
  initializerFeeVault: PublicKey;
  feeRate: number;
  adminKey: PublicKey;
  customState?: Uint8Array;
  fundState?: FundState;
}

class ExecutePoolAction extends PoolRequestInner {
  index: number;
  amount: BN;
}

export const schema = new Map<Function, any>([
  [
    PoolRequest,
    {
      kind: 'struct',
      fields: [
        ['tag', 'u64'],
        ['index', 'u8'],
        ['inner', PoolRequestInner],
      ],
    },
  ],
  [
    InitializePoolRequest,
    {
      kind: 'struct',
      fields: [
        ['nonce', 'u8'],
        ['assetsLength', 'u8'],
        ['poolName', 'string'],
        ['feeRate', 'u32'],
        ['customData', ['u8']],
      ],
    },
  ],
  [
    InitializeFundData,
    {
      kind: 'struct',
      fields: [
        ['slippageDivider', 'u64'],
        ['assetWeights', ['u32']],
        ['initialSupply', 'u64'],
      ],
    },
  ],
  [
    OptionAddress,
    {
      kind: 'struct',
      fields: [['value', [33]]],
    },
  ],
  [
    PubKey,
    {
      kind: 'struct',
      fields: [['value', [32]]],
    },
  ],
  [
    AssetInfo,
    {
      kind: 'struct',
      fields: [
        ['mint', PubKey],
        ['vaultAddress', PubKey],
      ],
    },
  ],
  [
    PoolState,
    {
      kind: 'struct',
      fields: [
        ['tag', 'u64'],
        ['poolTokenMint', PubKey],
        ['assets', [AssetInfo]],
        ['vaultSigner', PubKey],
        ['vaultSignerNonce', 'u8'],
        ['accountParams', [ParamDesc]],
        ['name', 'string'],
        ['lqdFeeVault', PubKey],
        ['initializerFeeVault', PubKey],
        ['feeRate', 'u32'],
        ['adminKey', OptionAddress],
        ['customState', ['u8']],
      ],
    },
  ],
  [
    ParamDesc,
    {
      kind: 'struct',
      fields: [
        ['address', PubKey],
        ['writable', 'u8'],
      ],
    },
  ],
  [
    FundState,
    {
      kind: 'struct',
      fields: [
        ['paused', 'u8'],
        ['slippageDivider', 'u64'],
        ['assetWeights', ['u32']],
        ['basicAsset', AssetInfo],
      ],
    },
  ],
  [
    ExecutePoolAction,
    {
      kind: 'struct',
      fields: [
        ['index', 'u8'],
        ['amount', 'u64'],
      ],
    },
  ],
]);

export class Fund {
  fundProgramId: PublicKey;

  constructor(connection: Connection, fundProgramId: PublicKey) {
    Object.assign(this, {
      connection,
      fundProgramId,
    });
  }

  static createInitializeInstruction(
    fundProgramId: PublicKey,
    fundAccount: PublicKey,
    fundTokenMint: PublicKey,
    fundVaultAccounts: PublicKey[],
    fundVaultAuthority: PublicKey,
    lqdFeeAccount: PublicKey,
    initializerFeeAccount: PublicKey,
    sysvar: PublicKey,
    fundAdminAccount: PublicKey,
    initialSupplyFundTokenAccount: PublicKey,
    fundBasicTokenVaultAccount: PublicKey,
    tokenProgramId: PublicKey,
    nonce: number,
    poolName: string,
    feeRate: number,
    assetWeights: number[],
    initialSupply: BN,
    slippageDivider: BN,
  ): TransactionInstruction {
    const keys = [
      { pubkey: fundAccount, isSigner: false, isWritable: true },
      { pubkey: fundTokenMint, isSigner: false, isWritable: true },
      ...fundVaultAccounts.map((acc) => ({ pubkey: acc, isSigner: false, isWritable: true })),
      { pubkey: fundVaultAuthority, isSigner: false, isWritable: false },
      { pubkey: lqdFeeAccount, isSigner: false, isWritable: false },
      { pubkey: initializerFeeAccount, isSigner: false, isWritable: false },
      { pubkey: sysvar, isSigner: false, isWritable: false },
      { pubkey: fundAdminAccount, isSigner: false, isWritable: true },
      { pubkey: initialSupplyFundTokenAccount, isSigner: false, isWritable: true },
      { pubkey: fundBasicTokenVaultAccount, isSigner: false, isWritable: false },
      { pubkey: tokenProgramId, isSigner: false, isWritable: false },
    ];

    const initializeFundData = new InitializeFundData({
      slippageDivider,
      assetWeights,
      initialSupply,
    });

    const serializedInitializeFundData = borsh.serialize(schema, initializeFundData);

    const initializePoolRequest = new InitializePoolRequest({
      nonce,
      assetsLength: fundVaultAccounts.length,
      poolName,
      feeRate,
      customData: serializedInitializeFundData,
    });

    const poolRequest = new PoolRequest({
      tag: PoolRequestTag,
      index: 0,
      inner: initializePoolRequest,
    });

    const serializedData = borsh.serialize(schema, poolRequest);

    return new TransactionInstruction({
      keys,
      programId: fundProgramId,
      data: Buffer.from(
        serializedData.buffer,
        serializedData.byteOffset,
        serializedData.byteLength,
      ),
    });
  }

  static createExecuteInstruction(
    fundProgramId: PublicKey,
    fundAccount: PublicKey,
    fundTokenMint: PublicKey,
    fundVaultAccounts: PublicKey[],
    fundVaultAuthority: PublicKey,
    userPoolTokenAccount: PublicKey,
    userAssetsAccounts: PublicKey[],
    authorityUserAccounts: PublicKey,
    lqdFeeAccount: PublicKey,
    initializerFeeAccount: PublicKey,
    refferFeeVault: PublicKey,
    tokenProgramId: PublicKey,
    amount: BN,
  ): TransactionInstruction {
    const keys = [
      { pubkey: fundAccount, isSigner: false, isWritable: true },
      { pubkey: fundTokenMint, isSigner: false, isWritable: true },
      ...fundVaultAccounts.map((acc) => ({ pubkey: acc, isSigner: false, isWritable: true })),
      { pubkey: fundVaultAuthority, isSigner: false, isWritable: false },
      { pubkey: userPoolTokenAccount, isSigner: false, isWritable: true },
      ...userAssetsAccounts.map((acc) => ({ pubkey: acc, isSigner: false, isWritable: true })),
      { pubkey: authorityUserAccounts, isSigner: true, isWritable: false },
      { pubkey: lqdFeeAccount, isSigner: false, isWritable: true },
      { pubkey: initializerFeeAccount, isSigner: false, isWritable: true },
      { pubkey: refferFeeVault, isSigner: false, isWritable: true },
      { pubkey: tokenProgramId, isSigner: false, isWritable: false },
    ];

    const executePoolAction = new ExecutePoolAction({
      index: 0,
      amount,
    });

    const poolRequest = new PoolRequest({
      tag: PoolRequestTag,
      index: 2,
      inner: executePoolAction,
    });

    const serializedData = borsh.serialize(schema, poolRequest);

    return new TransactionInstruction({
      keys,
      programId: fundProgramId,
      data: Buffer.from(
        serializedData.buffer,
        serializedData.byteOffset,
        serializedData.byteLength,
      ),
    });
  }

  static decodePoolState(buffer: Buffer): PoolState {
    const poolState = borsh.deserialize(schema, PoolState, buffer);

    const fundState = borsh.deserialize(schema, FundState, Buffer.from(poolState.customState));
    poolState.fundState = fundState;

    return poolState;
  }
}
