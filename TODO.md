# DarkReach – Roadmap & API Specification

## 1. TODO List,  Rendre DarkReach OP

1.1 Fondation technique (obligatoire)
- Définir un protocole commun (shared crate)
- formats JSON / MsgPack
- enums de commandes
- TLS obligatoire (mTLS idéalement)
- Identité unique par agent (UUID + fingerprint)

---

1.2 C2 Server – Core Features
- Enregistrement des agents
- Heartbeat / check-in périodique
- Queue de commandes par agent
- Stockage des résultats
- Gestion des attack patterns YAML
- Auth CLI → C2 (token / cert)
- Logs structurés (JSON)

---

1.3 Agents – Core Features
- Bootstrap & enrollment
- Exécution de commandes atomiques
- Exécution de workflows (YAML)
- Remontée des résultats
- Gestion du timeout / retry
- Auto-update (optionnel mais très OP)

---

1.4 CLI – Fonctions de base
- Connexion au C2
- Liste des agents
- Détails d’un agent
- Envoi de commandes
- Lancement d’attack patterns
- Récupération des résultats
- Export report (JSON / MD)

---

1.5 Sécurité & OPSEC
- Chiffrement bout-en-bout
- Rotation des clés
- Rate limiting C2
- Randomisation des intervals agents
- Obfuscation minimale des payloads
- Mode lab / demo / prod

---

1.6 Reporting & Exploitabilité
- Normalisation des résultats
- Tags MITRE ATT&CK
- Timeline des actions
- Export Markdown / JSON
- Preuve (PCAP, stdout, artefacts)

---

## 2. API C2 – Routes Complètes (MVP + scalable)

Base URL

```
/api/v1
```

### 2.1 Authentification

?


### 2.2 Agents

```
POST /agents/register
```

Agent → C2 (1ère connexion)

```json
{
  "agent_id": "uuid",
  "hostname": "target-01",
  "os": "linux",
  "arch": "x86_64",
  "ip": "10.0.0.12",
  "fingerprint": "sha256"
}
```

---

```
POST /agents/heartbeat
```

Agent → C2

```json
{
  "agent_id": "uuid",
  "status": "idle",
  "capabilities": ["scan", "exec", "exfil"]
}
```

---

```
GET /agents
```
CLI → C2
Liste des agents

---

```
GET /agents/{agent_id}
```

Détails d’un agent

---

```
DELETE /agents/{agent_id}
```

Désenregistrer / kill agent

---

### 2.3 Commandes

```
POST /commands
```

CLI → C2
Envoi d’une commande

```json
{
  "agent_id": "uuid",
  "command": "port_scan",
  "args": {
    "target": "192.168.1.0/24"
  }
}
```

---

```
GET /agents/{agent_id}/commands
```

Agent → C2
Récupération des commandes en attente

---

```
POST /commands/{command_id}/result
```

Agent → C2
Retour d’exécution
```json
{
  "status": "success",
  "stdout": "...",
  "stderr": "",
  "artifacts": []
}
```


### 2.4 Attack Patterns (YAML)

```
POST /patterns
```

Upload d’un pattern YAML

```yml
id: lateral-move-scan
steps:
  - action: port_scan
    args:
      target: 10.0.0.0/24
  - action: vuln_scan
    args:
      severity: high
```

---

```
GET /patterns
```

Liste des patterns disponibles

---

```
POST /patterns/{id}/execute
```

Exécution sur un ou plusieurs agents

```json
{
  "agents": ["uuid1", "uuid2"]
}
```

---

2.5 Résultats

```
GET /results
```

Tous les résultats

---

```
GET /results/{agent_id}
```

Résultats par agent

---

```
GET /results/{execution_id}
```

Résultat d’un workflow



## 3. CLI, Commandes attendues

```sh
darkreach login
darkreach agent list
darkreach agent info <id>
darkreach agent exec <id> <command>
darkreach pattern list
darkreach pattern run <pattern> --agents all
darkreach results list
darkreach report generate <execution_id> --format md
```

## 4. YAML Attack Pattern – Spec recommandée

```yaml
id: internal-recon
metadata:
  author: DarkReach
  mitre:
    - TA0007
steps:
  - id: scan
    action: port_scan
    args:
      target: "{{agent.ip}}"
  - id: enum
    action: service_enum
    depends_on: scan
```