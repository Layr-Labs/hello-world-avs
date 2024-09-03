import { Signer, providers, utils } from 'ethers';

export class RemoteSigner extends Signer {
  private readonly address: string;
  readonly provider: providers.Provider;
  private readonly remoteSigningEndpoint: string;

  constructor(address: string, provider: providers.Provider, remoteSigningEndpoint: string) {
    super();
    this.address = address;
    this.provider = provider;
    this.remoteSigningEndpoint = remoteSigningEndpoint;
  }

  async getAddress(): Promise<string> {
    return this.address;
  }

  async signMessage(message: string | utils.Bytes): Promise<string> {
    if (typeof(message) === "string") {
      return this.signMessageHash(message);
    } else {
      const messageHash = utils.solidityKeccak256(["string"], [message])
      return this.signMessageHash(messageHash);
    }
  }

  async signTransaction(transaction: utils.Deferrable<providers.TransactionRequest>): Promise<string> {
    // Implement the logic to send the transaction to your remote signing service
    // and return the signed transaction
    const tx = {
      from: transaction.from,
      to: transaction.to,
      value: transaction.value,
      gas: transaction.gasLimit?.toString(),
      maxPriorityFeePerGas: transaction.maxPriorityFeePerGas?.toString(),
      maxFeePerGas: transaction.maxFeePerGas?.toString(),
      nonce: transaction.nonce,
      data: transaction.data,
    }
    const signedTransaction = await callJsonRpcEndpoint(
        this.remoteSigningEndpoint,
        "eth_signTransaction",
        [tx]
    );
  
    return signedTransaction;
  }

  connect(provider: providers.Provider): Signer {
    return new RemoteSigner(this.address, provider, this.remoteSigningEndpoint);
  }

  private async signMessageHash(messageHash: string): Promise<string> {
    // Implement the logic to send the message hash to your remote signing service
    // and return the signature
    const signature = await callJsonRpcEndpoint(
        this.remoteSigningEndpoint,
        "eth_sign",
        [this.address, messageHash]
    );

    return signature;
  }
}

interface JsonRpcRequest {
  jsonrpc: string;
  method: string;
  params: any[];
  id: number;
}

interface JsonRpcResponse {
  jsonrpc: string;
  result?: any;
  error?: {
      code: number;
      message: string;
  };
  id: number;
}

async function callJsonRpcEndpoint(
  url: string,
  method: string,
  params: any[] = []
): Promise<any> {
  const request: JsonRpcRequest = {
      jsonrpc: "2.0",
      method,
      params,
      id: 1, // You might want to generate a unique id for each request
  };

  const response = await fetch(url, {
      method: "POST",
      headers: {
          "Content-Type": "application/json",
      },
      body: JSON.stringify(request),
  });

  if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
  }

  const jsonResponse: JsonRpcResponse = await response.json();

  if (jsonResponse.error) {
      throw new Error(`JSON-RPC error: ${jsonResponse.error.message}`);
  }

  return jsonResponse.result;
}