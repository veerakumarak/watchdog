export const API_BASE = 'http://localhost:8080/api'; // Adjust if your backend is on a different port (e.g., http://localhost:3000)
export const EXTERNAL_API_BASE_URL = 'http://localhost:8080/api'; // Adjust if your backend is on a different port (e.g., http://localhost:3000)

// Helper to handle response errors

export async function handleResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
        const errorBody = await response.text();
        throw new Error(errorBody || `HTTP error! status: ${response.status}`);
    }
    return response.json();
}
