import test from 'node:test'
import assert from 'node:assert/strict'
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const scriptDir = path.dirname(fileURLToPath(import.meta.url))
const root = path.resolve(scriptDir, '..', '..')

const ledgerPath = path.join(root, '.gsd/milestones/M057/slices/S01/reconciliation-ledger.json')
const auditPath = path.join(root, '.gsd/milestones/M057/slices/S01/reconciliation-audit.md')

test('M057 S01 publishes the final reconciliation ledger and audit surfaces', () => {
  assert.ok(fs.existsSync(ledgerPath), 'T03 must publish .gsd/milestones/M057/slices/S01/reconciliation-ledger.json')
  assert.ok(fs.existsSync(auditPath), 'T03 must publish .gsd/milestones/M057/slices/S01/reconciliation-audit.md')
})
