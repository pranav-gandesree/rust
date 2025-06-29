# this api should fail
Invoke-RestMethod -Uri http://localhost:1111/ -Method Get

# get basic vehicle data
Invoke-RestMethod -Uri http://localhost:1111/example -Method Get

# post basic vehicle data
Invoke-RestMethod -Uri http://localhost:1111/example -Method Post

