function About() {
  return (
    <div className="min-h-screen bg-slate-950 text-slate-50">
      <div className="mx-auto max-w-3xl px-4 py-10">
        <div className="rounded-2xl border border-slate-800 bg-slate-900/40 p-6 shadow-xl shadow-black/20 sm:p-10">
          <h1 className="text-3xl font-semibold tracking-tight mb-6">About Wata-Board</h1>
          
          <div className="space-y-6 text-slate-300">
            <p>
              Wata-Board is a decentralized utility payment platform built on the Stellar blockchain. 
              We enable users to pay their utility bills (water, electricity) securely and transparently 
              using cryptocurrency.
            </p>
            
            <div className="border-t border-slate-800 pt-6">
              <h2 className="text-xl font-semibold text-slate-100 mb-4">Our Mission</h2>
              <p>
                To democratize utility payments by leveraging blockchain technology for fast, 
                secure, and borderless transactions. We believe everyone deserves access to 
                reliable and transparent utility payment systems.
              </p>
            </div>
            
            <div className="border-t border-slate-800 pt-6">
              <h2 className="text-xl font-semibold text-slate-100 mb-4">How It Works</h2>
              <ul className="list-disc list-inside space-y-2">
                <li>Connect your Freighter wallet</li>
                <li>Enter your meter number</li>
                <li>Input the amount you want to pay</li>
                <li>Confirm the transaction on the blockchain</li>
              </ul>
            </div>
            
            <div className="border-t border-slate-800 pt-6">
              <h2 className="text-xl font-semibold text-slate-100 mb-4">Technology</h2>
              <p>
                Built on Stellar/Soroban smart contracts, Wata-Board ensures immutable and 
                verifiable payment records. All transactions are recorded on the blockchain, 
                providing complete transparency.
              </p>
            </div>
            
            <div className="rounded-full bg-sky-500/10 px-3 py-1 text-xs font-medium text-sky-300 ring-1 ring-inset ring-sky-500/20 inline-block mt-4">
              Testnet Mode
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default About;
