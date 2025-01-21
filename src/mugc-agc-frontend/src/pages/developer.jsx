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
    const [ledgerItems, setLedgerItems] = useState<Array<WorkLoadLedgerItem>>([]);

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
    React.useEffect(() => {
        if (getPrincipal()) {
            query_workflow_ledger_by_principal_id(getPrincipal()).then((result) => {
                if ('Ok' in result) {
                    setLedgerItems(result.Ok);
                } else if ('Err' in result) {
                    setProcessInfo('Error querying ledger: ' + result.Err);
                }
            }).catch((error) => {
                setProcessInfo('Error: ' + error.message);
            });
        }
    }, [getPrincipal()]);

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

const queryAssets = (pagenum) => {
    let newData = [{
        id: 1,
        c1: 'column1',
        c2: 'column2',
        c3: 'column3',
        c4: 'column4'
    }]
    setAssetsData(newData);
    let p = assetsPage;
    p.pageNum = pagenum
    // p.totalPage = 10
    setAssetsPage(p);
}

return (
  <div className="uv-container-1 container-subpg pg-dashboard">
    <div className={style.develop_form_ctx}>
        <div className={style.block}>
            <div className={style.title}>Paste your Workflow:</div>
            <div className={style.ipt_area}>
                <textarea
                    className={style.textareainput}
                    value={workflowJson}
                    onChange={(e) => setWorkflowJson(e.target.value)}
                    placeholder="Enter workflow JSON"
                />
            </div>
            <div className={`${style.btn} btn-link-1`} onClick={handleWorkFlowSubmit}>Submit WorkFlow</div>
        </div>
        
        <div className="sub-block-split"></div>

        <div className={style.block}>
            <div className={style.title}>Sample Output:</div>
            <div className={style.ipt_area}>
                <textarea
                    className={style.textareainput}
                    value={sampleOutput}
                    onChange={(e) => setSampleOutput(e.target.value)}
                    placeholder="Enter sample output"
                />
            </div>
            <div className={`${style.btn} btn-link-1`} onClick={handleContracrSubmit}>Submit and Wait Identity Process</div>
        </div>
        
        <div className="sub-block-split"></div>

        {/* <div className="submit-section">
            <div className="process-info">
                {processInfo}
            </div>
        </div> */}
  <div className={style.myassets}>
    <div className={style.block_title}>My Assets</div>
    {
    assetsData.length === 0 ? 
    <div className="nodata">
      <div className="nodata-img"></div>
      <div className="nodata-txt">No data</div>
    </div>
    :
    <div className="tbl-paged">
      <div className="tbl">
        <div className={`tbl-r tbl-r-title ${style.assets_row}`}>
          <div className="tbl-cell-title">column1</div>
          <div className="tbl-cell-title">column2</div>
          <div className="tbl-cell-title">column3</div>
          <div className="tbl-cell-title">column4</div>
        </div>
      {assetsData.map((el) => (
        <div key={el.id} className={`tbl-r ${style.assets_row}`}>
          <div className="tbl-cell">{el.c1}</div>
          <div className="tbl-cell">{el.c2}</div>
          <div className="tbl-cell">{el.c3}</div>
          <div className="tbl-cell">{el.c4}</div>
        </div>
      ))}
      </div>
      <Paging pageNum={assetsPage.pageNum} totalPage={assetsPage.totalPage} queryHandler={queryAssets} />
    </div>
    }
  </div>

    </div>
  </div>                  
     
);
};


export default Developer;