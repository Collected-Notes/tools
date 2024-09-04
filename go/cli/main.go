package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"strings"
)

const baseURL = "https://api.collectednotes.com"

// Read the API token from the file
func getAPIToken() (string, error) {
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", fmt.Errorf("failed to get home directory: %w", err)
	}

	tokenPath := homeDir + "/.collected-notes"
	tokenBytes, err := ioutil.ReadFile(tokenPath)
	if err != nil {
		return "", fmt.Errorf("error reading API token from ~/.collected-notes: %w", err)
	}

	// Trim any whitespace or newline characters from the token
	token := strings.TrimSpace(string(tokenBytes))
	return token, nil
}

// Make an HTTP GET request
func getRequest(url string, token string) ([]byte, error) {
	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Accept", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("error: %s", resp.Status)
	}

	return ioutil.ReadAll(resp.Body)
}

// Make an HTTP POST request
func postRequest(url string, token string, data map[string]interface{}) ([]byte, error) {
	jsonData, err := json.Marshal(data)
	if err != nil {
		return nil, err
	}

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Accept", "application/json")
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusCreated && resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("error: %s", resp.Status)
	}

	return ioutil.ReadAll(resp.Body)
}

// Make an HTTP PUT request
func putRequest(url string, token string, data map[string]interface{}) ([]byte, error) {
	jsonData, err := json.Marshal(data)
	if err != nil {
		return nil, err
	}

	req, err := http.NewRequest("PUT", url, bytes.NewBuffer(jsonData))
	if err != nil {
		return nil, err
	}
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Accept", "application/json")
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("error: %s", resp.Status)
	}

	return ioutil.ReadAll(resp.Body)
}

// Make an HTTP DELETE request
func deleteRequest(url string, token string) ([]byte, error) {
	req, err := http.NewRequest("DELETE", url, nil)
	if err != nil {
		return nil, err
	}
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Accept", "application/json")

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("error: %s", resp.Status)
	}

	return ioutil.ReadAll(resp.Body)
}

// Main function to handle CLI commands
func main() {
	// Determine the base URL based on the presence of the --dev flag
	baseURL := "https://api.collectednotes.com"
	args := os.Args[1:] // Get all arguments after the program name

	// Check if the --dev flag is present
	devMode := false
	for i, arg := range args {
		if arg == "--dev" {
			devMode = true
			// Remove the --dev flag from the arguments list
			args = append(args[:i], args[i+1:]...)
			break
		}
	}

	if devMode {
		baseURL = "http://localhost:3000"
	}
	
	token, err := getAPIToken()
	if err != nil {
		fmt.Println(err)
		return
	}

	if token == "" {
		fmt.Println("API token not found. Please ensure the token is stored in ~/.collected-notes")
		return
	}

	if len(os.Args) < 2 {
		fmt.Println("Usage: <command> [arguments]")
		fmt.Println("Available commands: get-sites, get-site <site_path>, create-site <site_path> <name>")
		return
	}

	command := os.Args[1]

	switch command {
	case "get-sites":
		data, err := getRequest(baseURL+"/sites", token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "get-site":
		if len(os.Args) < 3 {
			fmt.Println("Usage: get-site <site_path>")
			return
		}
		sitePath := os.Args[2]
		data, err := getRequest(baseURL+"/sites/"+sitePath, token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "create-site":
		if len(os.Args) < 4 {
			fmt.Println("Usage: create-site <site_path> <name>")
			return
		}
		sitePath := os.Args[2]
		name := os.Args[3]
		data, err := postRequest(baseURL+"/sites", token, map[string]interface{}{"site_path": sitePath, "name": name})
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "update-site":
		if len(os.Args) < 4 {
			fmt.Println("Usage: update-site <site_path> <name> [headline] [about] [domain]")
			return
		}
		sitePath := os.Args[2]
		name := os.Args[3]
		data := map[string]interface{}{"site_path": sitePath, "name": name}
		if len(os.Args) > 4 {
			data["headline"] = os.Args[4]
		}
		if len(os.Args) > 5 {
			data["about"] = os.Args[5]
		}
		if len(os.Args) > 6 {
			data["domain"] = os.Args[6]
		}
		response, err := putRequest(baseURL+"/sites/"+sitePath, token, data)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(response))

	case "delete-site":
		if len(os.Args) < 3 {
			fmt.Println("Usage: delete-site <site_path>")
			return
		}
		sitePath := os.Args[2]
		data, err := deleteRequest(baseURL+"/sites/"+sitePath, token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "get-notes":
		if len(os.Args) < 3 {
			fmt.Println("Usage: get-notes <site_path>")
			return
		}
		sitePath := os.Args[2]
		data, err := getRequest(baseURL+"/sites/"+sitePath+"/notes", token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "create-note":
		if len(os.Args) < 5 {
			fmt.Println("Usage: create-note <site_path> <body> <visibility>")
			return
		}
		sitePath := os.Args[2]
		body := os.Args[3]
		visibility := os.Args[4]
		data, err := postRequest(baseURL+"/sites/"+sitePath+"/notes", token, map[string]interface{}{"body": body, "visibility": visibility})
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "get-note":
		if len(os.Args) < 4 {
			fmt.Println("Usage: get-note <site_path> <note_path>")
			return
		}
		sitePath := os.Args[2]
		notePath := os.Args[3]
		data, err := getRequest(baseURL+"/sites/"+sitePath+"/notes/"+notePath, token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "update-note":
		if len(os.Args) < 5 {
			fmt.Println("Usage: update-note <site_path> <note_path> <body> <visibility>")
			return
		}
		sitePath := os.Args[2]
		notePath := os.Args[3]
		body := os.Args[4]
		visibility := os.Args[5]
		data, err := putRequest(baseURL+"/sites/"+sitePath+"/notes/"+notePath, token, map[string]interface{}{"body": body, "visibility": visibility})
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "delete-note":
		if len(os.Args) < 4 {
			fmt.Println("Usage: delete-note <site_path> <note_path>")
			return
		}
		sitePath := os.Args[2]
		notePath := os.Args[3]
		data, err := deleteRequest(baseURL+"/sites/"+sitePath+"/notes/"+notePath, token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "get-links-from-note":
		if len(os.Args) < 4 {
			fmt.Println("Usage: get-links-from-note <site_path> <note_path>")
			return
		}
		sitePath := os.Args[2]
		notePath := os.Args[3]
		data, err := getRequest(baseURL+"/sites/"+sitePath+"/notes/"+notePath+"/links", token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "get-note-body-as-html":
		if len(os.Args) < 4 {
			fmt.Println("Usage: get-note-body-as-html <site_path> <note_path>")
			return
		}
		sitePath := os.Args[2]
		notePath := os.Args[3]
		data, err := getRequest(baseURL+"/sites/"+sitePath+"/notes/"+notePath+"/body", token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "get-note-as-markdown":
		if len(os.Args) < 4 {
			fmt.Println("Usage: get-note-as-markdown <site_path> <note_path>")
			return
		}
		sitePath := os.Args[2]
		notePath := os.Args[3]
		data, err := getRequest(baseURL+"/sites/"+sitePath+"/notes/"+notePath+".md", token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "get-note-as-plaintext":
		if len(os.Args) < 4 {
			fmt.Println("Usage: get-note-as-plaintext <site_path> <note_path>")
			return
		}
		sitePath := os.Args[2]
		notePath := os.Args[3]
		data, err := getRequest(baseURL+"/sites/"+sitePath+"/notes/"+notePath+".txt", token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	case "search-notes":
		if len(os.Args) < 4 {
			fmt.Println("Usage: search-notes <site_path> <term> [mode]")
			return
		}
		sitePath := os.Args[2]
		term := os.Args[3]
		mode := "exact"
		if len(os.Args) > 4 {
			mode = os.Args[4]
		}
		data, err := getRequest(baseURL+"/sites/"+sitePath+"/notes/search?term="+term+"&mode="+mode, token)
		if err != nil {
			fmt.Println("Error:", err)
			return
		}
		fmt.Println(string(data))

	default:
		fmt.Printf("Unknown command: %s\n", command)
		fmt.Println("Available commands: get-sites, get-site <site_path>, create-site <site_path> <name>, update-site <site_path> <name> [headline] [about] [domain], delete-site <site_path>, get-notes <site_path>, create-note <site_path> <body> <visibility>, get-note <site_path> <note_path>, update-note <site_path> <note_path> <body> <visibility>, delete-note <site_path> <note_path>, get-links-from-note <site_path> <note_path>, get-note-body-as-html <site_path> <note_path>, get-note-as-markdown <site_path> <note_path>, get-note-as-plaintext <site_path> <note_path>, search-notes <site_path> <term> [mode]")
	}
}
