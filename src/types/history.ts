export interface HistorySummary {
  totalCount: number
  favoriteCount: number
}

export interface HistoryQuery {
  search?: string
  favoriteOnly?: boolean
  limit: number
  offset: number
}
