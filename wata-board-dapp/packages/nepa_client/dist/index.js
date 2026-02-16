import { Buffer } from "buffer";
import { Client as ContractClient, Spec as ContractSpec, } from "@stellar/stellar-sdk/contract";
export * from "@stellar/stellar-sdk";
export * as contract from "@stellar/stellar-sdk/contract";
export * as rpc from "@stellar/stellar-sdk/rpc";
if (typeof window !== "undefined") {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
}
export const networks = {
    testnet: {
        networkPassphrase: "Test SDF Network ; September 2015",
        contractId: "CD7QD2PP2CMPOH453VIEPVN6J2UMB3U6PNTR6OK5KZ7B3WYEVE2QWZLW",
    }
};
export class Client extends ContractClient {
    options;
    static async deploy(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options) {
        return ContractClient.deploy(null, options);
    }
    constructor(options) {
        super(new ContractSpec(["AAAAAAAAAAAAAAAIcGF5X2JpbGwAAAACAAAAAAAAAAhtZXRlcl9pZAAAABAAAAAAAAAABmFtb3VudAAAAAAABAAAAAA=",
            "AAAAAAAAAAAAAAAOZ2V0X3RvdGFsX3BhaWQAAAAAAAEAAAAAAAAACG1ldGVyX2lkAAAAEAAAAAEAAAAE"]), options);
        this.options = options;
    }
    fromJSON = {
        pay_bill: (this.txFromJSON),
        get_total_paid: (this.txFromJSON)
    };
}
