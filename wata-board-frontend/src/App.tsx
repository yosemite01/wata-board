import { BrowserRouter as Router, Routes, Route, Link, useLocation } from 'react-router-dom';
import { useState } from 'react';
import { isConnected, requestAccess, signTransaction } from "@stellar/freighter-api";
import * as NepaClient from './contracts';
import About from './pages/About';
import Contact from './pages/Contact';
import Rate from './pages/Rate';

function Navigation() {
  const location = useLocation();

  const isActive = (path: string) => {
    return location.pathname === path ? 'text-sky-400' : 'text-slate-300 hover:text-slate-100';
  };

  return (
    <nav className="border-b border-slate-800 bg-slate-900/60 backdrop-blur-sm sticky top-0 z-50">
      <div className="mx-auto max-w-3xl px-4 py-4">
        <div className="flex items-center justify-between">
          <Link to="/" className="text-xl font-semibold tracking-tight text-slate-100">
            Wata-Board
          </Link>
          <div className="flex items-center gap-6 text-sm">
            <Link to="/" className={`transition ${isActive('/')}`}>Pay Bill</Link>
            <Link to="/about" className={`transition ${isActive('/about')}`}>About</Link>
            <Link to="/contact" className={`transition ${isActive('/contact')}`}>Contact</Link>
            <Link to="/rate" className={`transition ${isActive('/rate')}`}>Rate Us</Link>
          </div>
        </div>
      </div>
    </nav>
  );
}

function Home() {
  const [meterId, setMeterId] = useState('');
  const [amount, setAmount] = useState('');
  const [status, setStatus] = useState('');

  const handlePayment = async () => {
    if (!(await isConnected())) {
      setStatus("Please install Freighter Wallet extension!");
      return;
    }

    try {
      if (!meterId.trim()) {
        setStatus('Please enter a meter number.');
        return;
      }

      const parsedAmount = Number(amount);
      if (!Number.isFinite(parsedAmount) || parsedAmount <= 0) {
        setStatus('Please enter a valid amount greater than 0.');
        return;
      }

      const amountU32 = Math.floor(parsedAmount);
      if (amountU32 !== parsedAmount) {
        setStatus('Amount must be a whole number.');
        return;
      }

      if (amountU32 > 0xffffffff) {
        setStatus('Amount is too large.');
        return;
      }

      setStatus("Connecting to wallet...");
      await requestAccess();

      const client = new NepaClient.Client({
        ...NepaClient.networks.testnet,
        rpcUrl: 'https://soroban-testnet.stellar.org:443',
      });

      setStatus('Preparing transaction... Please approve in Freighter.');

      const tx = await client.pay_bill({
        meter_id: meterId.trim(),
        amount: amountU32,
      });

      await tx.signAndSend({
        signTransaction: async (transactionXdr) => {
          const signedTx = await signTransaction(transactionXdr, { network: 'TESTNET' });
          return signedTx as string;
        },
      });

      setStatus(`Success! Payment confirmed for ${meterId.trim()}.`);
    } catch (err: any) {
      console.error(err);
      setStatus(`Payment failed: ${err?.message || 'Check console.'}`);
    }
  };

  return (
    <div className="min-h-screen bg-slate-950 text-slate-50">
      <div className="mx-auto max-w-3xl px-4 py-10">
        <div className="rounded-2xl border border-slate-800 bg-slate-900/40 p-6 shadow-xl shadow-black/20 sm:p-10">
          <div className="flex items-start justify-between gap-6">
            <div>
              <h1 className="text-3xl font-semibold tracking-tight">Wata-Board</h1>
              <p className="mt-2 max-w-prose text-sm text-slate-300">
                Decentralized utility payments on Stellar (Testnet).
              </p>
            </div>
            <div className="rounded-full bg-sky-500/10 px-3 py-1 text-xs font-medium text-sky-300 ring-1 ring-inset ring-sky-500/20">
              Testnet
            </div>
          </div>

          <div className="mt-8 grid gap-4 sm:grid-cols-2">
            <label className="grid gap-2">
              <span className="text-sm font-medium text-slate-200">Meter number</span>
              <input
                className="h-11 rounded-xl border border-slate-800 bg-slate-950/50 px-4 text-sm text-slate-100 outline-none ring-sky-500/30 placeholder:text-slate-500 focus:ring-4"
                placeholder="e.g. METER-123"
                value={meterId}
                onChange={(e) => setMeterId(e.target.value)}
              />
            </label>

            <label className="grid gap-2">
              <span className="text-sm font-medium text-slate-200">Amount</span>
              <input
                className="h-11 rounded-xl border border-slate-800 bg-slate-950/50 px-4 text-sm text-slate-100 outline-none ring-sky-500/30 placeholder:text-slate-500 focus:ring-4"
                placeholder="Whole number"
                type="number"
                min={1}
                step={1}
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
              />
            </label>
          </div>

          <div className="mt-6 flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
            <button
              onClick={handlePayment}
              className="inline-flex h-11 items-center justify-center rounded-xl bg-sky-500 px-5 text-sm font-semibold text-white shadow-lg shadow-sky-500/20 ring-1 ring-inset ring-white/10 transition hover:bg-sky-400 focus:outline-none focus:ring-4 focus:ring-sky-500/30"
            >
              Pay bill
            </button>
            <p className="text-xs text-slate-400">
              Requires Freighter extension.
            </p>
          </div>

          <div className="mt-8 rounded-xl border border-slate-800 bg-slate-950/40 p-4">
            <div className="text-xs font-semibold uppercase tracking-wide text-slate-400">Status</div>
            <div className="mt-2 text-sm text-slate-100">{status || 'Ready.'}</div>
          </div>
        </div>
      </div>
    </div>
  );
}

function App() {
  return (
    <Router>
      <Navigation />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/about" element={<About />} />
        <Route path="/contact" element={<Contact />} />
        <Route path="/rate" element={<Rate />} />
      </Routes>
    </Router>
  );
}

export default App;