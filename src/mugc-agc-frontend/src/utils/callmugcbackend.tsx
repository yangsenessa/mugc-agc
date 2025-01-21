/**
 * Utility functions for interacting with the MUGC AGC backend canister
 * @module callmugcbackend
 */

/** Development mode flag based on local network status */
 
/**
 * Identity provider URL for authentication
 * Local development: http://{canister_id}.localhost:4943
 * Production: https://identity.ic0.app
 */



import { mugc_agc_backend } from "declarations/mugc-agc-backend";
import { UploaderPowContractInput } from "declarations/mugc-agc-backend/mugc-agc-backend.did";
import { isLocalNet } from '@/utils/env';

// Mode
const development = isLocalNet();
// Identity provider URL
const IDENTITY_PROVIDER = development
  ? `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943` 
  : "https://identity.ic0.app";


/**
 * Stores workflow data in the backend canister
 * @param principalId - The principal ID of the user
 * @param workflowData - The workflow data to store
 * @returns A promise that resolves to either {Ok: string} on success or {Err: string} on failure
 * @throws Will return an error object if the backend call fails
 */
export async function storeWorkflowData(principalId: string, workflowData: string): Promise<{ Ok: string } | { Err: string }> {
    try {
        const result = await mugc_agc_backend.store_workflow_data(principalId, workflowData);
        return result;
    } catch (error) {
        return { Err: `Failed to store workflow data: ${error.message}` };
    }
}

/**
 * Stores an uploader POW (Proof of Work) contract in the backend canister
 * @param input - The UploaderPowContractInput object containing contract details
 * @returns A promise that resolves to either {Ok: null} on success or {Err: string} on failure
 * @throws Will return an error object if the backend call fails
 */
export async function storeUploaderPowContract(input: UploaderPowContractInput): Promise<{ Ok: null } | { Err: string }> {
    try {
        const result = await mugc_agc_backend.store_uploader_pow_contract(input);
        return result;
    } catch (error) {
        return { Err: `Failed to store uploader POW contract: ${error.message}` };
    }
}