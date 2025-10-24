# 🛡️ Aether VPN

**Aether VPN** is a secure, enterprise-grade VPN solution built as part of the **Aether Office Suite**.  
Inspired by modern solutions like **NordVPN** and **Tailscale**, it combines **privacy, scalability, and simplicity** with a fully open and extensible architecture.

---

## 🚀 Overview

Aether VPN is designed to provide a **zero-configuration, high-security VPN platform** for both individuals and enterprises.  
It features a clean **Next.js + TypeScript client**, powered by a centralized backend API hosted on  
`api.skygenesisenterprise.com`.

### Key Principles

- **Security-first design** — end-to-end encryption via WireGuard or OpenVPN.
- **Unified API** — all VPN logic centralized under `/api/v1/vpn`.
- **Cross-platform client** — built with Next.js and TypeScript.
- **Enterprise integration** — single sign-on, device management, and role-based access control.
- **Extensible core** — modular backend, ready for custom deployment or self-hosting.

---

## 🧠 Architecture Overview

```

Client (Next.js / TS)
│
│  HTTPS + JWT / OAuth2
▼
Backend API: api.skygenesisenterprise.com
│
├── Auth & Team Management
├── Device & Tunnel Control
├── Server Registry
├── Key Distribution (WireGuard / OpenVPN)
└── Metrics & Monitoring

```

---

## 🧩 API Structure

All VPN logic is handled via REST routes under `/api/v1/vpn`.

| Endpoint | Method | Description |
|-----------|---------|-------------|
| `/auth/login` | POST | User or enterprise authentication |
| `/users/me` | GET | Retrieve current user info & roles |
| `/devices` | GET / POST / DELETE | Register, list, or remove devices |
| `/servers` | GET | Fetch available VPN servers |
| `/tunnels/connect` | POST | Establish VPN tunnel |
| `/tunnels/disconnect` | POST | Close VPN tunnel |
| `/keys/:deviceId` | GET | Secure key distribution |
| `/metrics` | GET | VPN usage statistics & server metrics |

---

## 🧱 Folder Structure

```

/pages/api/v1/vpn/
├── auth/
│     ├── login.ts
│     ├── refresh.ts
├── users/
│     ├── me.ts
├── devices/
│     ├── index.ts
│     ├── [id].ts
├── servers/
│     ├── index.ts
│     ├── [id].ts
├── tunnels/
│     ├── connect.ts
│     ├── disconnect.ts
├── keys/
│     ├── [deviceId].ts
└── metrics/
├── index.ts

````

---

## ⚙️ Tech Stack

- **Frontend / Client** → [Next.js](https://nextjs.org/) + TypeScript  
- **Backend API** → Next.js API Routes (REST)  
- **Protocol Layer** → WireGuard / OpenVPN  
- **Auth** → JWT / OAuth2  
- **Database** → PostgreSQL or MongoDB (via Prisma ORM)  
- **Cloud Deployment** → Vercel, Fly.io, or self-hosted Docker containers  

---

## 🧩 Environment Setup

```bash
# Clone the repository
git clone https://github.com/aether-office/aether-vpn.git
cd aether-vpn

# Install dependencies
npm install

# Run the development server
npm run dev

# Access the app
http://localhost:3000
````

### Required Environment Variables

```
NEXT_PUBLIC_API_URL=https://api.skygenesisenterprise.com
DATABASE_URL=postgresql://user:pass@localhost:5432/aethervpn
JWT_SECRET=your_jwt_secret
WIREGUARD_KEY_PATH=/etc/aethervpn/keys/
```

---

## 🏢 Enterprise & Pro Features (Planned)

* **Team & Role Management**
* **Advanced Audit Logs**
* **Magic DNS & Split Tunneling**
* **Zero-Trust Access Control**
* **Custom Region Deployment**
* **WebSocket Realtime Metrics**

---

## 🧰 Roadmap

| Stage                  | Description                               | Status        |
| ---------------------- | ----------------------------------------- | ------------- |
| **MVP**                | Authentication, basic tunnel control      | ✅ In progress |
| **v1.0**               | Multi-region servers, key management      | 🛠️ Planned   |
| **Enterprise Edition** | Role-based access, DNS, metrics dashboard | ⏳ Future      |
| **Mobile Clients**     | iOS / Android integration                 | 🔮 Future     |

---

## 🧑‍💻 Contributing

We welcome open-source contributions!
If you want to contribute, please:

1. Fork this repository
2. Create a new feature branch
3. Submit a pull request with clear documentation

---

## 🌌 Part of the Aether Office Ecosystem

Aether VPN is a component of the **Aether Office Suite**, a collection of open, privacy-focused productivity and network tools:

* 🧭 **Aether Desk** — Knowledge and workspace platform
* 💾 **Aether Wallet** — Secure digital identity and key management
* 🎵 **Aether Wave** — Decentralized music platform
* 🏬 **Aether Store** — Open-source universal app marketplace

---


## 🛡️ License

Aether VPN is released under the **Apache 2.0 License**.
See [`LICENSE`](./LICENSE) for more information.