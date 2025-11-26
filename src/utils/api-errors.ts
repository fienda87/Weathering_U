/**
 * Custom error class for API responses
 */
export class ApiErrorResponse extends Error {
  constructor(
    public status: number,
    public message: string,
    public details?: any
  ) {
    super(message)
    this.name = 'ApiErrorResponse'
  }

  /**
   * Check if error is a 404 Not Found error
   */
  static is404(error: unknown): error is ApiErrorResponse {
    return error instanceof ApiErrorResponse && error.status === 404
  }

  /**
   * Check if error is a 400 Bad Request error
   */
  static is400(error: unknown): error is ApiErrorResponse {
    return error instanceof ApiErrorResponse && error.status === 400
  }

  /**
   * Check if error is a 500 Internal Server Error
   */
  static is500(error: unknown): error is ApiErrorResponse {
    return error instanceof ApiErrorResponse && error.status === 500
  }
}

/**
 * Handle API response and throw ApiErrorResponse on error
 */
export async function handleApiResponse(response: Response) {
  if (!response.ok) {
    const data = await response.json().catch(() => ({}))
    throw new ApiErrorResponse(
      response.status,
      data.error || response.statusText,
      data
    )
  }
  return response.json()
}
