import React, { useState } from 'react';
import './developer.module.scss';
import style from './developer.module.scss'
import {storeWorkflowData,storeUploaderPowContract} from "@/utils/callmugcbackend"
import { useAcountStore } from '@/stores/user';
import {reConnectPlug} from '@/utils/icplug';



const Developer = () => {
    const [workflowJson, setWorkflowJson] = useState('');
    const [nodeId, setNodeId] = useState('');
    const [paramValue, setParamValue] = useState('');
    const [processInfo, setProcessInfo] = useState('');
    const [uploaderPowContract, setUploaderPowContract] = useState({
        sample_output: '',
        identity_gusserr_limit: 0.0,
        workflow_id: ''
    });
    const [sampleOutput, setSampleOutput] = useState('');
    const [identityGusserrLimit, setIdentityGusserrLimit] = useState(0.3);
    const [workflowId, setWorkflowId] = useState('');

    const { setUserByPlugWallet, getPrincipal, getWalletType } = useAcountStore();

    React.useEffect(() => {
        if (!getPrincipal()) {
            reConnectPlug()
                .then((principal_id) => {
                    if (principal_id) {
                        setUserByPlugWallet(principal_id);
                    }
                })
                .catch((e) => {
                    console.log('reConnectPlug exception!', e);
                });
        }
    }, []);


    const handleWorkFlowSubmit = (e) => {
        e.preventDefault();
        try {
            //const parsedWorkflow = JSON.parse(workflowJson);
            storeWorkflowData(getPrincipal(), workflowJson).then((result) => {
                console.log('storeWorkflowData done, result:', result);
                if ('Ok' in result) {
                    setProcessInfo('Workflow stored successfully,workflowid : ' + result.Ok);
                    setWorkflowId(result.Ok);
                } else if ('Err' in result) {
                    setProcessInfo('Error storing workflow: ' + result.Err);
                } else {
                    setProcessInfo('Unexpected response from backend');
                }
            }
        )            
            // Add your workflow processing logic here
        } catch (error) {
            setProcessInfo('Error: Invalid JSON format');
        }
    };

const handleContracrSubmit = (e) => {
    e.preventDefault();
    
    // Build uploaderPowContract object with current state values
    const contract = {
        sample_output: sampleOutput,
        identity_gusserr_limit: identityGusserrLimit,
        workflow_id: workflowId
    };
    setUploaderPowContract(contract);

    // Call backend function to store the contract
    storeUploaderPowContract(contract).then((result) => {
        if ('Ok' in result) {
            setProcessInfo('Contract stored successfully');
        } else if ('Err' in result) {
            setProcessInfo('Error storing contract: ' + result.Err);
        } else {
            setProcessInfo('Unexpected response from backend');
        }
    }).catch((error) => {
        setProcessInfo('Error: ' + error.message);
    });
};

    return (
        <div className="${style.develop_form_ctx}">
            <div className="{style.title}">
                <h3>Paste your Workflow:</h3>
                <textarea
                    className="{style.textareainput}"
                    value={workflowJson}
                    onChange={(e) => setWorkflowJson(e.target.value)}
                    placeholder="Enter workflow JSON"
                />
            </div>
            <div className={style.split_line}>
                <hr />
            </div>
            <div className="submit-section">
                <button className="submit-button" onClick={handleWorkFlowSubmit}>
                    Submit WorkFlow
                </button>
            </div>     

            <div className={style.title}>
                <h3>Sample Output:</h3>
                <textarea
                    className={style.textareainput}
                    value={sampleOutput}
                    onChange={(e) => setSampleOutput(e.target.value)}
                    placeholder="Enter sample output"
                />
            </div>
              <div className={style.split_line}>
                <hr />
            </div>

            <div className="submit-section">
                <button className="submit-button" onClick={handleContracrSubmit}>
                    Submit and Wait Identity Process
                </button>
                <div className="process-info">
                    {processInfo}
                </div>
            </div>

        </div>
                            
         
    );
};

export default Developer;