# Wata-Board Project

A decentralized utility payment platform built on **Stellar/Soroban** blockchain. This project enables users to pay utility bills (water, electricity) using cryptocurrency.

## Project Structure

```
wata-board/
├── wata-board-dapp/          # Node.js backend dapp
│   ├── src/index.ts          # Main payment processing script
│   └── packages/             # Contract client libraries
│       ├── nepa_client/      # Original client
│       └── nepa_client_v2/   # Updated client (v2)
│
└── wata-board-frontend/      # React + TypeScript + Vite frontend
    ├── src/
    │   ├── App.tsx           # Main payment UI
    │   └── contracts/        # Frontend contract bindings
    └── ...
```

## Quick Start

### Prerequisites

- **Node.js** (LTS recommended, v18+)
- **npm** or **yarn**
- **Freighter Wallet** browser extension (for frontend)
- Stellar account with testnet XLM

### 1. Install Dependencies

```bash
# Install frontend dependencies
cd wata-board-frontend
npm install

# Install dapp dependencies  
cd ../wata-board-dapp
npm install
```

### 2. Run Frontend (Development)

```bash
cd wata-board-frontend
npm run dev
```

Open the URL shown (usually `http://localhost:5173`)

### 3. Run Dapp (Backend)

```bash
cd wata-board-dapp
npx ts-node src/index.ts
```

## Smart Contract

- **Contract ID**: `CDRRJ7IPYDL36YSK5ZQLBG3LICULETIBXX327AGJQNTWXNKY2UMDO4DA`
- **Network**: Stellar Testnet
- **RPC URL**: `https://soroban-testnet.stellar.org`

### Contract Methods

| Method | Description | Parameters |
|--------|-------------|------------|
| `pay_bill` | Record a utility payment | `meter_id` (string), `amount` (u32) |
| `get_total_paid` | Get total paid for a meter | `meter_id` (string) |

## Environment Variables

Create `.env` files for sensitive configuration:

### Frontend (.env)
```
VITE_CONTRACT_ID=CDRRJ7IPYDL36YSK5ZQLBG3LICULETIBXX327AGJQNTWXNKY2UMDO4DA
VITE_RPC_URL=https://soroban-testnet.stellar.org
```

### Dapp (.env)
```
SECRET_KEY=your_secret_key_here
CONTRACT_ID=CDRRJ7IPYDL36YSK5ZQLBG3LICULETIBXX327AGJQNTWXNKY2UMDO4DA
RPC_URL=https://soroban-testnet.stellar.org
```

⚠️ **Security**: Never commit `.env` files with real keys to git!

## Testing

```bash
# Frontend tests
cd wata-board-frontend
npm run test

# Dapp tests  
cd wata-board-dapp
npm test
```

## Build for Production

```bash
# Build frontend
cd wata-board-frontend
npm run build

# Output will be in dist/ folder
```

## CI/CD Pipeline

The project includes GitHub Actions workflows for:
- **Lint & Type Check**: Validates code quality
- **Build**: Ensures production builds succeed
- **Test**: Runs test suite

See `.github/workflows/` for configuration.

## Scripts Reference

| Script | Command | Description |
|--------|---------|-------------|
| Dev server | `npm run dev` | Start Vite dev server |
| Build | `npm run build` | Create production build |
| Lint | `npm run lint` | Run ESLint |
| Preview | `npm run preview` | Preview production build |

## Troubleshooting

### Common Issues

1. **"Cannot find module" errors**
   - Run `npm install` in the correct directory
   - Check that `packages/` symlink exists if needed

2. **Freighter not connecting**
   - Ensure Freighter extension is installed
   - Set Freighter network to "Testnet"
   - Refresh the page after connecting

3. **Transaction failures**
   - Verify you have testnet XLM in your account
   - Check contract ID is correct
   - Ensure meter_id is a valid string

## Network Configuration

| Network | Passphrase | RPC URL |
|---------|------------|---------|
| Testnet | Test SDF Network ; September 2015 | https://soroban-testnet.stellar.org |
| Mainnet | Public Global Stellar Network ; September 2015 | https://soroban.stellar.org |

## Contributing

1. Fork the repository
2. Create a feature branch
3. Run tests: `npm test`
4. Submit a pull request

## License

ISC
