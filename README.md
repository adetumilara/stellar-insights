# Ndii Intelligence Dashboard

**A Stellar Analytics Engine that quantifies payment reliability and liquidity health — so wallets, apps, and anchors can make payments with confidence.**

[![React](https://img.shields.io/badge/React-18-blue)](https://react.dev) [![TypeScript](https://img.shields.io/badge/TypeScript-5.8-blue)](https://www.typescriptlang.org) [![Vite](https://img.shields.io/badge/Vite-5.4-blueviolet)](https://vitejs.dev) [![Stellar](https://img.shields.io/badge/Stellar-Network-brightgreen)](https://stellar.org)

---

## 🎯 The Problem

Stellar is widely used for **stablecoins, cross-border payments, asset issuance, and remittances**, but *speed alone isn't enough* — **payment reliability and liquidity health are real challenges**:

- 📉 **Which asset corridors consistently succeed vs fail?**
- 💧 **Is there enough liquidity in major payment paths?**
- ⚓ **Are certain anchors or assets bottlenecks in transfers?**
- 📊 **Are markets efficient, stable, or unreliable under stress?**

**Current tools** show raw transactions; few show *how well payments actually work in practice*.

---

## 💡 The Solution

Ndii Intelligence Dashboard provides **deep insights into Stellar payment network performance**, enabling wallets, developers, businesses, and anchors to:

✅ **Predict payment success likelihood** before sending  
✅ **Optimize routing paths** for reliability  
✅ **Quantify real-world payment reliability** (not just TPS)  
✅ **Identify liquidity bottlenecks** and improve provisioning  
✅ **Understand ecosystem risk** and health trends  

---

## 📊 What You Get

### 🎯 Core Intelligence Features

| Feature | Impact |
|---------|--------|
| **Payment Reliability Metrics** | % successful vs attempted payments across corridors |
| **Corridor Health Scoring** | Average slippage, bid-ask spread, liquidity depth analysis |
| **Anchor Performance Tracking** | Weighted reliability scores and failure rate monitoring |
| **Liquidity Dynamics** | TVL trends, active liquidity over time, stress testing |
| **Payment Latency Analysis** | Median confirmation times and settlement speed trends |
| **Heatmap Visualization** | See which asset pairs and corridors are most/least reliable |

### 🛣️ Key Pages

**📈 Dashboard**
- Real-time KPIs: Success rate, active corridors, liquidity depth, settlement speed
- Payment volume trends and patterns
- Corridor reliability heatmap
- Top performing assets ranking

**🛣️ Corridors Page**
- Deep dive into payment corridors (asset pair routes)
- Success rate and slippage metrics
- Volume trends and historical analysis
- Identify fragile vs robust routing paths

**⚓ Anchors Page**
- Anchor/issuer reliability scorecards
- Asset portfolio per anchor
- Transaction success rates and failure monitoring
- Health status indicators (green/yellow/red)

---

## 👥 Who Benefits

| User Type | Value |
|-----------|-------|
| **Wallets & Apps** | Predict payment success before sending; suggest optimal routes |
| **Businesses & Remittance Services** | Avoid failed transactions; build reliable payment infrastructure |
| **Anchors & Issuers** | See liquidity provisioning gaps; prioritize high-adoption markets |
| **DeFi Builders** | Understand ecosystem efficiency and routing reliability |
| **Compliance & Regulators** | Spot congested corridors; assess ecosystem risk |

---

## 🏗️ Full-Stack Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                Frontend (React + TypeScript)                │
│         Dashboard | Corridors | Anchors | Analytics         │
└──────────────────────────┬─────────────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────────────┐
│              REST API (Node.js + Express)                   │
│    /api/payments | /api/corridors | /api/anchors            │
└──────────────────────────┬─────────────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────────────┐
│      Backend: Analytics Engine (Rust Core)                  │
│  Ingests RPC → Computes Metrics → Generates Snapshots       │
│  ┌────────────────────────────────────────────────────────┐ │
│  │ • Payment Success Rate  • Settlement Latency           │ │
│  │ • Liquidity Depth       • Corridor Scores              │ │
│  │ • Anchor Reliability    • TVL Trends                   │ │
│  └────────────────────────────┬──────────────────────────┘ │
│                                │                            │
│                                ▼                            │
│                    Hash Snapshot & Push to Chain            │
└──────────────────────────┬─────────────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────────────┐
│     Smart Contract (Soroban, Rust) - On-Chain Anchor       │
│  Stores: Analytics Hashes | Timestamps | Verification Data  │
│  Functions: submit_snapshot() | get_snapshot() | verify()   │
└──────────────────────────┬─────────────────────────────────┘
                           │
                           ▼
┌──────────────────────────────────────────────────────────────┐
│              Data Sources (Stellar Blockchain)              │
│    RPC | Horizon API | Ledger | Order Books | Trades       │
└──────────────────────────────────────────────────────────────┘
```

### Key Insight: Separation of Concerns
- **Frontend**: Visualization & UX
- **API Layer**: Data exposure & routing
- **Backend (Rust)**: Computation, scoring, intelligence
- **Smart Contract**: Verification, immutability, trustlessness
- **Blockchain**: Immutable source of truth

---

## 🚀 Full Tech Stack

**Frontend (React SPA)**
- React 18 with TypeScript for type-safe components
- Vite 5 for lightning-fast builds and HMR
- shadcn-ui for consistent, accessible UI components
- Tailwind CSS for responsive design
- Recharts for financial data visualization
- TanStack React Query for efficient data fetching
- React Router v6 for seamless navigation

**Backend (Analytics Engine)**
- Rust for high-performance data processing
- Stellar SDK for blockchain data ingestion
- PostgreSQL/TimescaleDB for time-series metrics
- Redis for caching and real-time data
- Bull Queue for job processing

**Smart Contract (On-Chain Verification)**
- Soroban (Stellar's smart contract platform)
- Rust for contract development
- Cryptographic hashing for data integrity
- Timestamped snapshots for audit trails

**DevOps & Infrastructure**
- Docker for containerization
- GitHub Actions for CI/CD
- Vercel/Netlify for frontend hosting
- AWS or DigitalOcean for backend

**Developer Experience**
- TypeScript for type safety (frontend)
- ESLint for code quality
- Vitest + React Testing Library for testing
- Hot module reloading during development

---

## 🎬 Getting Started

### Prerequisites

**For Frontend:**
- Node.js 18+ ([install with nvm](https://github.com/nvm-sh/nvm))
- npm or yarn

**For Backend (Rust Analytics Engine):**
- Rust 1.70+ ([install](https://rustup.rs/))
- PostgreSQL 14+ (or Docker)
- Cargo (comes with Rust)

### Full-Stack Setup

This project has **two parts that run independently**:

#### 1️⃣ Start PostgreSQL Database

```bash
# Option A: Using Docker
docker run --name stellar-postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=stellar_insights \
  -p 5432:5432 \
  -d postgres:14

# Option B: Using local PostgreSQL
# Make sure PostgreSQL is running and create the database:
# createdb stellar_insights
```

#### 2️⃣ Build & Run Backend (Rust)

```bash
cd backend

# Run the server (migrations run automatically)
cargo run
```

You should see: `Server starting on 127.0.0.1:8080`

Test it:
```bash
curl http://localhost:8080/health
```

#### 3️⃣ Setup & Run Frontend (React + Next.js)

```bash
# From the root directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

The app will be available at `http://localhost:3000`

### Frontend Commands

```bash
npm run dev          # Start dev server
npm run build        # Production build
npm start            # Start production server
npm run lint         # Code quality check
```

### Backend Commands

```bash
cd backend

cargo run            # Run the server
cargo build          # Build binary
cargo test           # Run tests
cargo test -- --nocapture  # Tests with output
```

**⚠️ Important**: Both backend and frontend need to run simultaneously for the full app to work!

### Troubleshooting

**Backend won't build (Rust errors)**
```bash
cd backend
rustup update        # Update Rust toolchain
cargo clean          # Clean build artifacts
cargo build          # Rebuild
```

**Database connection failed**
```bash
# Verify PostgreSQL is running
docker ps | grep stellar-postgres

# Check backend logs
# Error should show in terminal output

# Reset database if needed
docker exec -it stellar-postgres \
  psql -U postgres -d stellar_insights -c \
  "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
```

**Port 8080 already in use (backend)**
```bash
# Check what's using port 8080
lsof -i :8080

# Change port in backend/.env
echo "SERVER_PORT=8081" >> backend/.env
```

**Port 3000 already in use (frontend)**
```bash
# Next.js will prompt for a different port
# Or specify it manually
npm run dev -- -p 3001
```

**Frontend can't connect to backend**
- Make sure backend is running: `curl http://localhost:8080/health`
- Check if frontend API calls use correct URL (`http://localhost:8080`)
- Check browser console for CORS errors

---

## 📁 Project Structure

```
stellar-insights/
├── src/
│   ├── pages/                # Main pages (Dashboard, Corridors, Anchors)
│   ├── components/
│   │   ├── dashboard/        # Dashboard visualizations
│   │   ├── layout/           # Navigation & layout
│   │   └── ui/               # Reusable shadcn-ui components
│   ├── data/
│   │   └── mockData.ts       # Mock metrics (ready for real API)
│   ├── hooks/                # React hooks (toast, mobile)
│   ├── lib/                  # Utilities
│   └── App.tsx               # Router configuration
├── public/                   # Static assets
├── vite.config.ts            # Vite configuration
├── tailwind.config.ts        # Tailwind theming
├── tsconfig.json             # TypeScript config
└── package.json              # Dependencies
```

---

## 📊 Core Metrics Explained

### Payment Reliability
Percentage of payments that successfully settle vs total attempted:
- Tracks corridor-specific success patterns
- Identifies failure-prone routes
- Helps predict payment outcomes

### Corridor Health Score
Composite metric measuring efficiency of asset pair routes:
- **Liquidity Depth**: Order book capacity for smooth trades
- **Slippage**: Price impact of trades (lower is better)
- **Success Rate**: Percentage of successful trades
- **Latency**: Median settlement time

### Anchor Performance Index
Reliability scoring for asset issuers:
- Success rate of transactions
- Failure frequency and patterns
- Asset diversity and adoption
- Health status (green/yellow/red)

### Liquidity Dynamics
Understanding of capital flow and availability:
- Total Value Locked (TVL) trends
- Active liquidity movements
- Stress testing scenarios
- Temporal patterns (peak vs off-peak)

---

## 🔌 Data Integration Ready

The dashboard uses mock data but is built to integrate with:

**Stellar RPC**
- Real-time ledger data
- Payment and transaction history
- Order book snapshots

**Data Indexing Services**
- Historical payment flow analysis
- Bitquery or similar indexing tools
- Custom event tracking

**Future Enhancements**
- Live data streaming
- Predictive analytics
- Alert systems
- Export capabilities

See [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for data integration details.

---

## 📈 MVP Scope & Roadmap

### Phase 1: Foundation (Current)
- [x] Dashboard infrastructure
- [x] Core UI components
- [x] Corridor analysis
- [x] Anchor tracking
- [x] Mock data structure
- [x] Documentation & architecture

### Phase 2: Backend & Smart Contract
- [x] Rust analytics engine ✅
- [x] Anchor metrics computation ✅
- [x] Database schema & persistence ✅
- [x] REST API endpoints ✅
- [x] Stellar RPC integration ✅ **NEW**
- [x] Data ingestion pipeline ✅ **NEW**
- [ ] Soroban smart contract deployment
- [ ] On-chain snapshot anchoring

### Phase 3: Real Data Integration
- [ ] Historical data indexing
- [ ] Real payment metrics
- [ ] Live liquidity feeds
- [ ] On-chain verification UI
- [ ] Audit trail display

### Phase 4: Advanced Analytics & Automation
- [ ] Predictive success scoring
- [ ] Optimal routing engine
- [ ] Alert system
- [ ] Custom reports
- [ ] Data export (CSV, JSON)

### Phase 5: Scale & Ecosystem
- [ ] Performance optimization
- [ ] Mobile-first improvements
- [ ] API webhooks for integrations
- [ ] Multi-language support
- [ ] Community contributions

---

## 🤝 Contributing

We welcome contributions! Here's how to get started:

```bash
# Create a feature branch
git checkout -b feature/your-feature

# Make your changes and commit
git commit -am "feat: describe your changes"

# Push and open a PR
git push origin feature/your-feature
```

**Guidelines:**
- Follow TypeScript best practices
- Ensure tests pass: `bun run test`
- Lint your code: `bun run lint`
- Add tests for new features

---

## 🚀 Deployment

### Build for Production
```bash
bun run build
```

Output goes to `dist/` directory.

### Deploy to Vercel (Recommended)
```bash
# Vercel auto-detects Vite
# Just connect your GitHub repo
```

### Other Platforms
- **Netlify** – Drag & drop `dist/` folder
- **AWS S3 + CloudFront** – Perfect for static SPA
- **Firebase Hosting** – Connect GitHub repo
- **GitHub Pages** – Simple static hosting

---

## 📖 Documentation

- [FEATURES.md](./docs/FEATURES.md) – Detailed feature documentation
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md) – System design & data flow
- [CONTRIBUTING.md](./CONTRIBUTING.md) – Development guidelines
- [API.md](./docs/API.md) – Endpoints & data structures

---

## 📊 Key Metrics At a Glance

**Current Dashboard Data (Example)**
- ✅ **Payment Success Rate**: 97.8%
- 🛣️ **Active Corridors**: 142 asset pair routes
- 💧 **Liquidity Depth**: $847.5M total
- ⚡ **Avg Settlement**: 4.2 seconds
- 📈 **7-Day Volume Trend**: Growing

---

## 🔗 Resources

- [Stellar Documentation](https://developers.stellar.org)
- [Stellar RPC API](https://developers.stellar.org/docs/data/apis/rpc)
- [Stellar Network Stats](https://stellar.expert)
- [Create React App Docs](https://react.dev)

---

## 📄 License

[Specify your license - e.g., MIT, Apache 2.0]

---

## 🎓 Portfolio Impact

**This project demonstrates advanced capabilities:**
- **Full-stack architecture**: Frontend (React/TS) → API (Node.js) → Backend (Rust) → Smart Contract (Soroban)
- **Blockchain integration**: Stellar RPC, Horizon, Soroban contracts
- **Financial metrics design**: Corridor scoring, anchor reliability, liquidity analysis
- **On-chain verification**: Trustless analytics via smart contracts
- **Production-grade practices**: TypeScript, Rust, testing, deployment
- **Systems thinking**: Data pipeline, metric computation, verification layer

**Credibility points:**
- Turns dashboard into on-chain infrastructure
- Makes analytics verifiable and tamper-proof
- Useful for foundations, anchors, compliance teams
- Addresses real Stellar ecosystem problems
- Clean separation of concerns (frontend/backend/contract)

**Great talking points for:**
- FinTech & Payment Systems roles
- Blockchain/Web3 Engineer positions
- Backend/Systems Engineer interviews
- Full-stack Rust + TypeScript teams
- Product management interviews
- Smart contract development roles

---

## �️ Smart Contract (Soroban, Rust)

### Purpose
Anchor analytics attestations on-chain for tamper-proof verification.

### What It Does
- **Stores periodic hashes** of computed analytics (corridor scores, anchor reliability)
- **Timestamps each update** for audit trails
- **Enables verification**: Anyone can verify off-chain analytics against on-chain proofs

### Contract Interface
```rust
pub fn submit_snapshot(hash: Bytes, epoch: u64)      // Store new snapshot
pub fn get_snapshot(epoch: u64) -> Bytes             // Retrieve specific snapshot
pub fn latest_snapshot() -> (Bytes, u64, u64)        // Get most recent (hash, epoch, timestamp)
pub fn verify_snapshot(hash: Bytes) -> bool          // Verify off-chain data
```

### Why It Matters
✅ **Makes analytics verifiable** – Anyone can audit results  
✅ **Enables trustless reporting** – No single point of failure  
✅ **Useful for** foundations, anchors, compliance teams, regulators  
✅ **Transforms product** – From dashboard to on-chain infrastructure  

---

## 🔧 Backend (Rust, Core Engine)

### Purpose
Compute, score, and publish Stellar network health metrics.

### Key Responsibilities
- **Ingest** Stellar RPC and Horizon data
- **Build** corridor and anchor metrics
- **Compute** deterministic analytics snapshots
- **Generate** verifiable hashes
- **Push** results to Soroban contract

### Data Flow
```
Stellar RPC → Analytics Engine → Snapshot Hash → Soroban Contract
       ↓              ↓                 ↓
  Ledger Data   Metrics Math    On-Chain Proof
                     ↓
              REST API → Frontend
```

### Metrics Computed
| Metric | Source | Purpose |
|--------|--------|---------|
| Payment Success Rate | Payment operations | Corridor reliability |
| Liquidity Depth | Order book snapshots | Available capacity |
| Settlement Latency | Transaction timestamps | Payment speed |
| Anchor Reliability | Success/failure counts | Issuer trust score |
| TVL Trends | Ledger state | Ecosystem health |

### Why Backend Is Central
- **All intelligence lives here** – Complex metrics computation
- **Smart contract only certifies** – Validates hashes, stores proofs
- **Frontend only visualizes** – Displays outputs, reads contract
- **Clean architecture** – Separation of concerns, testability

---

## 🌐 Why This Architecture Is Strong

### For Credibility
✅ **Smart contract adds verifiability**, not just a dashboard  
✅ **Rust backend does real data science** – Not trivial computation  
✅ **Clean separation of concerns** – Each layer has one job  
✅ **Production-ready** – Auditable, scalable, maintainable  

### For Users
✅ **Trust the data** – On-chain proofs verify accuracy  
✅ **Integrate easily** – Use REST API or read smart contract  
✅ **Historical audit trails** – Timestamped snapshots on-chain  
✅ **Network resilience** – No single point of failure  

---

## �📞 Support

For issues, questions, or suggestions:
- Open an [issue](https://github.com/Ndifreke000/stellar-insights/issues)
- Check [existing discussions](https://github.com/Ndifreke000/stellar-insights/discussions)
- Review [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for technical details
- Check [FEATURES.md](./docs/FEATURES.md) for use cases

---

**Built with ❤️ for the Stellar ecosystem**

*Making payment analytics verifiable, trustless, and on-chain.*
