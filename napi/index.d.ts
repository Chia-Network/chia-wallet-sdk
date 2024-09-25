/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Pair {
  first: ClvmPtr
  rest: ClvmPtr
}
export interface Output {
  value: ClvmPtr
  cost: bigint
}
export interface Curry {
  program: ClvmPtr
  args: Array<ClvmPtr>
}
export interface Coin {
  parentCoinInfo: Uint8Array
  puzzleHash: Uint8Array
  amount: bigint
}
export declare function toCoinId(coin: Coin): Uint8Array
export interface CoinSpend {
  coin: Coin
  puzzleReveal: Uint8Array
  solution: Uint8Array
}
export interface LineageProof {
  parentParentCoinInfo: Uint8Array
  parentInnerPuzzleHash?: Uint8Array
  parentAmount: bigint
}
export interface Nft {
  coin: Coin
  lineageProof: LineageProof
  info: NftInfo
}
export interface NftInfo {
  launcherId: Uint8Array
  metadata: NftMetadata
  metadataUpdaterPuzzleHash: Uint8Array
  currentOwner?: Uint8Array
  royaltyPuzzleHash: Uint8Array
  royaltyTenThousandths: number
  p2PuzzleHash: Uint8Array
}
export interface NftMetadata {
  editionNumber: bigint
  editionTotal: bigint
  dataUris: Array<string>
  dataHash?: Uint8Array
  metadataUris: Array<string>
  metadataHash?: Uint8Array
  licenseUris: Array<string>
  licenseHash?: Uint8Array
}
export interface ParsedNft {
  nftInfo: NftInfo
  innerPuzzle: ClvmPtr
}
export declare function parseNftInfo(clvm: ClvmAllocator, ptr: ClvmPtr): ParsedNft | null
export declare function parseUnspentNft(clvm: ClvmAllocator, parentCoin: Coin, parentPuzzleReveal: ClvmPtr, parentSolution: ClvmPtr, coin: Coin): Nft | null
export declare function spendNft(clvm: ClvmAllocator, nft: Nft, innerSpend: Spend): Array<CoinSpend>
export interface NftMint {
  metadata: NftMetadata
  p2PuzzleHash: Uint8Array
  royaltyPuzzleHash: Uint8Array
  royaltyTenThousandths: number
}
export interface MintedNfts {
  nfts: Array<Nft>
  coinSpends: Array<CoinSpend>
  parentConditions: Array<Uint8Array>
}
export declare function mintNfts(parentCoinId: Uint8Array, nftMints: Array<NftMint>): MintedNfts
export interface Spend {
  puzzle: ClvmPtr
  solution: ClvmPtr
}
export declare function spendP2Standard(clvm: ClvmAllocator, syntheticKey: Uint8Array, conditions: Array<ClvmPtr>): Spend
export declare function spendP2Singleton(clvm: ClvmAllocator, launcherId: Uint8Array, coinId: Uint8Array, singletonInnerPuzzleHash: Uint8Array): Spend
export declare function compareBytes(a: Uint8Array, b: Uint8Array): boolean
export declare function sha256(bytes: Uint8Array): Uint8Array
export declare function fromHexRaw(hex: string): Uint8Array
export declare function fromHex(hex: string): Uint8Array
export declare function toHex(bytes: Uint8Array): string
export declare class ClvmAllocator {
  constructor()
  deserialize(value: Uint8Array): ClvmPtr
  deserializeWithBackrefs(value: Uint8Array): ClvmPtr
  serialize(ptr: ClvmPtr): Uint8Array
  serializeWithBackrefs(ptr: ClvmPtr): Uint8Array
  treeHash(ptr: ClvmPtr): Uint8Array
  run(puzzle: ClvmPtr, solution: ClvmPtr, maxCost: bigint, mempoolMode: boolean): Output
  curry(ptr: ClvmPtr, args: Array<ClvmPtr>): ClvmPtr
  uncurry(ptr: ClvmPtr): Curry | null
  newList(values: Array<ClvmPtr>): ClvmPtr
  newPair(first: ClvmPtr, rest: ClvmPtr): ClvmPtr
  newAtom(value: Uint8Array): ClvmPtr
  newSmallNumber(value: number): ClvmPtr
  newBigInt(value: bigint): ClvmPtr
  list(ptr: ClvmPtr): Array<ClvmPtr>
  pair(ptr: ClvmPtr): Pair | null
  atom(ptr: ClvmPtr): Uint8Array | null
  atomLength(ptr: ClvmPtr): number | null
  smallNumber(ptr: ClvmPtr): number | null
  bigInt(ptr: ClvmPtr): bigint | null
}
export declare class ClvmPtr {
  static nil(): ClvmPtr
  isAtom(): boolean
  isPair(): boolean
}
