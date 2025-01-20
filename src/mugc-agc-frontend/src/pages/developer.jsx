import React, { useState } from 'react';
import './developer.module.scss';

const Developer = () => {
    const [workflowJson, setWorkflowJson] = useState('');
    const [nodeId, setNodeId] = useState('');
    const [paramValue, setParamValue] = useState('');
    const [processInfo, setProcessInfo] = useState('');

    const handleSubmit = (e) => {
        e.preventDefault();
        try {
            const parsedWorkflow = JSON.parse(workflowJson);
            setProcessInfo(`Workflow parsed successfully. Processing node ${nodeId}...`);
            // Add your workflow processing logic here
        } catch (error) {
            setProcessInfo('Error: Invalid JSON format');
        }
    };

    return (
        <div className="developer-container">
            <header className="developer-header">
                <h1>UnivOice ComfyUI Workflow Manager</h1>
            </header>
            
            <main className="developer-main">
                <form onSubmit={handleSubmit}>
                    <div className="input-group">
                        <label>Workflow JSON</label>
                        <textarea
                            value={workflowJson}
                            onChange={(e) => setWorkflowJson(e.target.value)}
                            placeholder="Paste your ComfyUI workflow JSON here..."
                            rows={10}
                            required
                        />
                    </div>

                    <button type="submit" className="submit-button">
                        Process Workflow
                    </button>

                    <div className="input-group">
                        <label>Node ID</label>
                        <input
                            type="text"
                            value={nodeId}
                            onChange={(e) => setNodeId(e.target.value)}
                            placeholder="Enter node ID"
                        />
                    </div>

                    <div className="input-group">
                        <label>Parameter Value</label>
                        <input
                            type="text"
                            value={paramValue}
                            onChange={(e) => setParamValue(e.target.value)}
                            placeholder="Enter parameter value (JSON, string, or number)"
                        />
                    </div>
                </form>

                <div className="process-info">
                    <h3>Process Information:</h3>
                    <p>{processInfo}</p>
                </div>

                <footer className="developer-footer">
                    <p>
                        Visit <a href="http://univoice.world" target="_blank" rel="noopener noreferrer">UnivOice.world</a> to learn more
                    </p>
                    <p className="claim-info">
                        Users can claim their processed workflows on the UnivOice platform
                    </p>
                </footer>
            </main>
        </div>
    );
};

export default Developer;