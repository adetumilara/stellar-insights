"use client"

import React, { useEffect, useState, useCallback } from 'react'
import {
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  CartesianGrid,
  Legend,
} from 'recharts'
import { MainLayout } from "@/components/layout"
import { Loader } from "lucide-react"

type Corridor = {
  id: string
  health: number
  successRate: number
}

type TopAsset = {
  asset: string
  volume: number
  tvl: number
}

type TimePoint = {
  ts: string
  successRate: number
  settlementMs: number
  tvl: number
}

type DashboardData = {
  totalSuccessRate: number
  activeCorridors: Corridor[]
  topAssets: TopAsset[]
  timeseries: TimePoint[]
}

export default function DashboardPage() {
  const [data, setData] = useState<DashboardData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  const fetchData = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const res = await fetch('/api/dashboard')
      if (!res.ok) throw new Error(`HTTP ${res.status}`)
      const json = await res.json()
      setData(json)
    } catch (err: any) {
      setError(err.message || 'Failed to load')
    } finally {
      setLoading(false)
    }
  }, [])

  useEffect(() => {
    fetchData()
    const id = setInterval(fetchData, 30_000) // refresh every 30s
    return () => clearInterval(id)
  }, [fetchData])

  return (
    <MainLayout>
      <div className="p-4 sm:p-6 lg:p-8 max-w-7xl mx-auto">
        <div className="mb-8">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">
                Network Dashboard
              </h1>
              <p className="text-gray-600 dark:text-gray-400">
                Overview of your Stellar payment network insights
              </p>
            </div>
            <button
              className="px-4 py-2 rounded-lg bg-blue-600 text-white text-sm hover:bg-blue-700 transition-colors"
              onClick={() => fetchData()}
              disabled={loading}
            >
              Refresh
            </button>
          </div>
        </div>

        {loading && !data && (
          <div className="flex items-center justify-center py-12">
            <Loader className="w-8 h-8 animate-spin text-blue-500" />
          </div>
        )}

        {error && (
          <div className="rounded-lg p-4 bg-rose-50 dark:bg-rose-900/20 text-rose-700 dark:text-rose-400 mb-6">
            Error: {error}
          </div>
        )}

        {data && (
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div className="col-span-1 bg-white dark:bg-slate-800 rounded-lg shadow border border-gray-200 dark:border-slate-700 p-6">
              <h2 className="text-sm text-gray-500 dark:text-gray-400 mb-3">Total Payment Success Rate</h2>
              <div className="mt-3 flex items-end gap-4">
                <div className="text-4xl font-bold text-gray-900 dark:text-white">
                  {(data.totalSuccessRate * 100).toFixed(2)}%
                </div>
                <div className="text-sm text-gray-500 dark:text-gray-400">(last 24h)</div>
              </div>
            </div>

            <div className="col-span-1 lg:col-span-2 bg-white dark:bg-slate-800 rounded-lg shadow border border-gray-200 dark:border-slate-700 p-6">
              <h2 className="text-sm text-gray-500 dark:text-gray-400 mb-3">Settlement Speed (ms) — last 24 points</h2>
              <div style={{ width: '100%', height: 220 }} className="mt-3">
                <ResponsiveContainer>
                  <LineChart data={data.timeseries}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                    <XAxis 
                      dataKey="ts" 
                      tickFormatter={(s) => new Date(s).getHours() + ':00'}
                      stroke="#6b7280"
                    />
                    <YAxis stroke="#6b7280" />
                    <Tooltip 
                      labelFormatter={(s) => new Date(s).toLocaleString()}
                      contentStyle={{ backgroundColor: 'white', border: '1px solid #e5e7eb' }}
                    />
                    <Legend />
                    <Line type="monotone" dataKey="settlementMs" stroke="#8884d8" dot={false} />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </div>

            <div className="col-span-1 lg:col-span-2 bg-white dark:bg-slate-800 rounded-lg shadow border border-gray-200 dark:border-slate-700 p-6">
              <h2 className="text-sm text-gray-500 dark:text-gray-400 mb-3">Liquidity Depth / TVL (24h)</h2>
              <div style={{ width: '100%', height: 240 }} className="mt-3">
                <ResponsiveContainer>
                  <LineChart data={data.timeseries}>
                    <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                    <XAxis 
                      dataKey="ts" 
                      tickFormatter={(s) => new Date(s).getHours() + ':00'}
                      stroke="#6b7280"
                    />
                    <YAxis stroke="#6b7280" />
                    <Tooltip 
                      labelFormatter={(s) => new Date(s).toLocaleString()}
                      contentStyle={{ backgroundColor: 'white', border: '1px solid #e5e7eb' }}
                    />
                    <Legend />
                    <Line type="monotone" dataKey="tvl" stroke="#82ca9d" dot={false} />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </div>

            <div className="col-span-1 bg-white dark:bg-slate-800 rounded-lg shadow border border-gray-200 dark:border-slate-700 p-6">
              <h2 className="text-sm text-gray-500 dark:text-gray-400 mb-3">Active Corridor Health</h2>
              <ul className="mt-3 space-y-3">
                {data.activeCorridors.map((c) => (
                  <li key={c.id} className="flex items-center justify-between">
                    <div>
                      <div className="font-medium text-gray-900 dark:text-white">{c.id}</div>
                      <div className="text-sm text-gray-500 dark:text-gray-400">Success: {(c.successRate * 100).toFixed(2)}%</div>
                    </div>
                    <div className="text-sm font-semibold text-gray-900 dark:text-white">
                      {(c.health * 100).toFixed(0)}%
                    </div>
                  </li>
                ))}
              </ul>
            </div>

            <div className="col-span-1 lg:col-span-2 bg-white dark:bg-slate-800 rounded-lg shadow border border-gray-200 dark:border-slate-700 p-6">
              <h2 className="text-sm text-gray-500 dark:text-gray-400 mb-3">Top-performing Assets</h2>
              <div className="mt-3 overflow-auto">
                <table className="w-full text-left text-sm">
                  <thead className="text-gray-500 dark:text-gray-400 text-xs uppercase">
                    <tr>
                      <th className="pb-2">Asset</th>
                      <th className="pb-2">Volume</th>
                      <th className="pb-2">TVL</th>
                    </tr>
                  </thead>
                  <tbody>
                    {data.topAssets.map((a) => (
                      <tr key={a.asset} className="border-t border-gray-200 dark:border-slate-700">
                        <td className="py-2 font-medium text-gray-900 dark:text-white">{a.asset}</td>
                        <td className="py-2 text-gray-700 dark:text-gray-300">{a.volume.toLocaleString()}</td>
                        <td className="py-2 text-gray-700 dark:text-gray-300">${a.tvl.toLocaleString()}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        )}
      </div>
    </MainLayout>
  )
}
