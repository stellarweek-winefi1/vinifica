# 🍷 VineFi – Invertí en Vino Premium de Forma Segura, Clara y Accesible

VineFi es una plataforma que permite digitalizar lotes de vino de inversión de Argentina y Chile, facilitando que bodegas y personas puedan participar en el mercado del vino premium sin fricciones, sin intermediarios innecesarios y con total transparencia.

---

## 1. Problem Statement

### What real-world problem are you solving?

El mercado de vino de inversión históricamente ofrece retornos atractivos (7-10% anual promedio), pero ha sido un mercado cerrado y exclusivo. Los principales problemas son:

- **Barreras de entrada altas**: Requiere inversiones mínimas de miles de dólares para acceder a vinos premium
- **Falta de liquidez**: Una vez invertido, el vino puede estar años guardado sin posibilidad de venderlo fácilmente
- **Falta de transparencia**: Precios opacos, comisiones ocultas, y poca visibilidad del estado real del inventario
- **Acceso limitado al capital para bodegas**: Dependencia de bancos y mayoristas, cobros lentos, y márgenes reducidos por intermediación

### For whom is this a problem?

**Para inversores:**
- Entusiastas del vino que quieren invertir pero no tienen el capital inicial requerido
- Inversores que buscan diversificar su portafolio con activos tangibles
- Personas que quieren exposición al mercado de vinos premium sin barreras técnicas o financieras

**Para bodegas:**
- Bodegas de Argentina y Chile que necesitan capital para expandirse sin endeudarse
- Productores que quieren acceso directo a una comunidad global de inversores
- Bodegas que buscan monetizar inventario almacenado de forma más eficiente

### Why is this problem urgent or important now?

- El vino de inversión ha crecido como activo alternativo en todo el mundo ($340B mercado global)
- Las nuevas generaciones buscan exposición a activos tangibles sin procesos financieros lentos o caros
- La digitalización de activos reales permite acceso global, fraccionamiento y trazabilidad sin complejidad
- Bodegas de Argentina y Chile están buscando nuevas fuentes de capital para expandirse sin endeudarse
- La tecnología blockchain permite transacciones rápidas, seguras y de bajo costo

---

## 2. Target User and User Need

### Who is your primary user?

**Usuario primario: Inversores individuales**
- Edad: 25-55 años
- Perfil: Entusiastas del vino, inversores que buscan diversificación, personas con interés en activos alternativos
- Ubicación: Global, con foco inicial en Latinoamérica, USA y Europa
- Nivel técnico: Bajo a medio - quieren usar la plataforma sin entender blockchain

**Usuario secundario: Bodegas**
- Bodegas de Argentina y Chile con vinos premium
- Bodegas que buscan capitalización sin intermediarios tradicionales

### What is their core need or pain point?

**Inversores:**
- Necesitan acceso a vinos premium con montos accesibles (desde $50)
- Requieren transparencia total en precios, disponibilidad y estado del inventario
- Necesitan liquidez para entrar y salir de inversiones cuando lo deseen
- Quieren un proceso simple, sin barreras técnicas o financieras complejas

**Bodegas:**
- Necesitan capital inmediato sin depender de bancos o distribuidores
- Buscan acceso directo a una comunidad global de inversores
- Requieren un proceso simple para digitalizar y monetizar su inventario

### How do they currently solve this?

**Inversores:**
- Compran botellas completas directamente de bodegas o distribuidores (requiere miles de dólares)
- Usan plataformas tradicionales de inversión en vino (altas comisiones, poca transparencia)
- Invierten a través de fondos de vino (barreras de entrada altas, falta de control)
- **Workaround actual**: No invierten o invierten montos muy pequeños sin acceso real a vinos premium

**Bodegas:**
- Venden a través de distribuidores y mayoristas (márgenes reducidos, cobros lentos)
- Obtienen préstamos bancarios (endeudamiento, procesos lentos)
- Venden directamente a consumidores finales (alcance limitado)
- **Workaround actual**: Mantienen inventario almacenado sin monetizar eficientemente

---

## 3. Solution Overview

### 3.1 Main Idea

VineFi es una plataforma que digitaliza lotes de vino premium en activos digitales fraccionados respaldados 1:1 por botellas físicas reales. Permite que bodegas conviertan su inventario en capital inmediato mediante la emisión de tokens digitales, y que inversores compren fracciones de estos lotes desde montos accesibles ($50+), con total transparencia, trazabilidad y liquidez a través de un mercado secundario.

**Core user journey:**

1. **Bodega**: Registra su bodega, sube información y documentación de un lote de vino, define precio por unidad, y digitaliza el lote creando tokens respaldados por botellas físicas
2. **Inversor**: Explora vinos disponibles en el mercado, selecciona un vino, elige cantidad de unidades a invertir, realiza el pago, y recibe tokens digitales en su wallet
3. **Gestión**: Inversores pueden ver su portafolio, trackear el valor de sus inversiones, y vender sus tokens en el mercado secundario cuando lo deseen

### 3.2 Why Stellar?

**Stellar Network es ideal para VineFi porque:**

- **Transacciones rápidas y baratas**: Comisiones de $0.00001 y liquidación en 3-5 segundos, perfecto para un marketplace de activos digitales
- **Stablecoins nativos**: USDC y otras stablecoins permiten pagos estables sin volatilidad de criptomonedas
- **Asset issuance**: Stellar permite crear y gestionar tokens personalizados (wine tokens) de forma nativa
- **Multi-currency support**: Facilita pagos desde diferentes países y monedas
- **Soroban Smart Contracts**: Para lógica compleja como:
  - Gestión de fraccionamiento de lotes
  - Marketplace secundario con matching de órdenes
  - Escrow y custodia de fondos
  - Verificación de ownership y transferencias
- **Wallets y on/off-ramps**: Integración fácil con wallets populares y servicios de conversión fiat/crypto
- **Transparencia y trazabilidad**: Todas las transacciones son públicas y verificables en la blockchain

**Elementos de Stellar que usaremos:**

- ✅ **Stellar Network**: Para transacciones de tokens y pagos
- ✅ **Soroban Smart Contracts**: Para lógica de negocio (marketplace, escrow, fraccionamiento)
- ✅ **Asset Issuance**: Creación de tokens representando fracciones de vino
- ✅ **Stablecoins (USDC)**: Para pagos estables
- ✅ **Stellar SDK**: Integración frontend/backend con la red
- ✅ **Wallets**: Integración con Freighter, WalletConnect, o wallets web

---

## 4. Core Features (Planned for the Hackathon)

### Feature 1: Digitalización de Vinos (Wine Digitization)
**What the user can do:**
- Bodegas pueden registrar información de un lote de vino (nombre, región, añada, cantidad de botellas, precio)
- Subir documentación (certificados de autenticidad, fotos, etc.)
- Digitalizar el lote creando tokens en Stellar que representan fracciones del inventario
- Ver el estado de sus lotes digitalizados

**How we will know if it's working:**
- Una bodega puede completar el formulario de digitalización
- Los tokens se crean exitosamente en Stellar para un lote
- El lote aparece en el marketplace disponible para inversión

### Feature 2: Marketplace de Vinos (Wine Marketplace)
**What the user can do:**
- Inversores pueden explorar vinos disponibles con información detallada
- Ver precio por unidad, retorno anual estimado, disponibilidad
- Buscar y filtrar vinos por región, precio, rating
- Ver detalles completos de cada vino antes de invertir

**How we will know if it's working:**
- Los vinos digitalizados aparecen en la página de mercado
- Los usuarios pueden buscar y filtrar vinos
- Cada vino muestra información correcta (precio, disponibilidad, etc.)

### Feature 3: Inversión en Vinos (Wine Investment)
**What the user can do:**
- Inversores pueden seleccionar un vino y especificar cantidad de unidades a comprar
- Ver resumen de inversión (unidades, precio total, retorno estimado)
- Conectar wallet de Stellar y realizar el pago
- Recibir tokens digitales en su wallet después del pago

**How we will know if it's working:**
- Un usuario puede completar una inversión end-to-end
- El pago se procesa correctamente en Stellar
- Los tokens se transfieren al wallet del usuario
- El inventario disponible se actualiza correctamente

### Feature 4: Portafolio de Inversiones (Investment Portfolio)
**What the user can do:**
- Inversores pueden ver todos sus vinos invertidos en un dashboard
- Ver valor actual de inversión, retorno, cantidad de unidades por vino
- Trackear el rendimiento de sus inversiones

**How we will know if it's working:**
- El portafolio muestra correctamente los tokens que el usuario posee
- Los valores y métricas se calculan correctamente
- La información se actualiza en tiempo real

### Feature 5: (Stretch Goal) Mercado Secundario (Secondary Market)
**What the user can do:**
- Inversores pueden listar sus tokens para venta en el mercado secundario
- Otros usuarios pueden comprar tokens de otros inversores
- Ver historial de transacciones y precios de mercado

**How we will know if it's working:**
- Un usuario puede listar tokens para venta
- Otro usuario puede comprar esos tokens
- La transacción se completa en Stellar y los tokens se transfieren

---

## 5. MVP Architecture (Initial Idea)

### Frontend
- **Framework**: Next.js 15 (React 19, TypeScript)
- **Styling**: Tailwind CSS
- **Animations**: Framer Motion
- **Stellar Integration**: 
  - `@stellar/stellar-sdk` para interacción con Stellar
  - `@stellar/freighter-api` para integración con Freighter wallet
  - WalletConnect para soporte de múltiples wallets

**Pages:**
- `/` - Homepage con hero, features, CTA
- `/mercado` - Marketplace de vinos disponibles
- `/mercado/[id]` - Página de inversión para un vino específico
- `/digitalizar` - Formulario para bodegas digitalizar vinos
- `/portafolio` - Dashboard de inversiones del usuario

### Backend / Services
- **Runtime**: Node.js
- **Framework**: Next.js API Routes (para MVP) o Express.js (si se necesita más complejidad)
- **Stellar Integration**: 
  - Servicio para crear y gestionar assets en Stellar
  - Servicio para procesar pagos y transferencias
  - Integración con Soroban para smart contracts

**API Endpoints (planned):**
- `POST /api/wines` - Crear nuevo vino digitalizado
- `GET /api/wines` - Listar vinos disponibles
- `GET /api/wines/[id]` - Obtener detalles de un vino
- `POST /api/invest` - Procesar inversión
- `GET /api/portfolio` - Obtener portafolio del usuario

### Smart Contracts (Soroban)
- **Wine Token Contract**: 
  - Gestión de emisión de tokens por lote de vino
  - Verificación de ownership
  - Transferencias de tokens
  
- **Marketplace Contract** (stretch goal):
  - Matching de órdenes de compra/venta
  - Escrow de fondos
  - Gestión de comisiones

**Lenguaje**: Rust (Soroban)

### Data / Storage
- **Database**: PostgreSQL o Supabase
  - Información de bodegas
  - Catálogo de vinos digitalizados
  - Usuarios y wallets
  - Historial de transacciones
  - Documentación de vinos (IPFS o storage similar)

- **Blockchain**: Stellar Network
  - Tokens de vino (assets)
  - Transacciones y ownership
  - Smart contracts (Soroban)

- **File Storage**: 
  - Imágenes y documentos de vinos (Supabase Storage, AWS S3, o IPFS)

### Architecture Diagram

```
┌─────────────┐
│   User      │
│  (Browser)  │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────┐
│         Frontend (Next.js)          │
│  - React Components                 │
│  - Stellar SDK Integration          │
│  - Wallet Connection (Freighter)    │
└──────┬──────────────────┬───────────┘
       │                  │
       │                  │
       ▼                  ▼
┌──────────────┐   ┌──────────────┐
│   Backend    │   │   Stellar    │
│  (API Routes)│   │   Network    │
│              │   │              │
│  - Business  │   │  - Assets    │
│    Logic     │◄──┤  - Payments  │
│  - Auth      │   │  - Soroban   │
│  - File      │   │    Contracts │
│    Upload    │   │              │
└──────┬───────┘   └──────────────┘
       │
       ▼
┌──────────────┐
│  PostgreSQL  │
│  / Supabase  │
│              │
│  - Wines     │
│  - Users     │
│  - Docs      │
└──────────────┘
```

---

## 6. Success Criteria for the Hackathon

By the end of Stellar Hack+, we will consider our MVP successful if:

- [ ] **A bodega can digitalize a wine lot**: Complete the digitalization form, upload documentation, and successfully create Stellar tokens representing the wine lot
- [ ] **An investor can browse available wines**: See a marketplace with at least 3 wines, with complete information (name, price, availability, return)
- [ ] **An investor can complete an investment**: Select a wine, choose number of units, connect Stellar wallet, make payment, and receive tokens in their wallet
- [ ] **An investor can view their portfolio**: See all their wine investments in a dashboard with correct token balances and investment details
- [ ] **We can demonstrate end-to-end flow**: From wine digitization to investment to portfolio view, all working on Stellar network
- [ ] **We can measure transactions**: Show transaction history on Stellar, verify token ownership, and demonstrate transparency

**Stretch Goals:**
- [ ] Secondary marketplace where users can buy/sell wine tokens
- [ ] Integration with multiple Stellar wallets (Freighter, WalletConnect)
- [ ] Real-time price updates and market data

---

## 7. Team

- **Team name**: VineFi Team

- **Members and roles**:
  - [Your Name] – Full-stack development, Stellar integration, smart contracts
  - (Add team members as needed)

- **Links**:
  - GitHub: (Add your repo link)
  - Demo: (Add demo link when available)
  - Stellar Account: (Add Stellar account if public)

---

## Getting Started

### Prerequisites
- Node.js 18+ 
- npm or yarn
- Stellar Testnet account (for development)

### Installation

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build

# Start production server
npm start
```

### Environment Variables

Create a `.env.local` file:

```env
NEXT_PUBLIC_STELLAR_NETWORK=testnet
NEXT_PUBLIC_STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
NEXT_PUBLIC_SOROBAN_RPC_URL=https://soroban-testnet.stellar.org
DATABASE_URL=your_database_url
```

### Stellar Setup

1. Create a Stellar testnet account
2. Fund it with testnet lumens
3. Configure your environment variables
4. Deploy Soroban contracts (when ready)

---

## License

MIT
