package main

func main() {
	var morse [37]string

	morse[0] = ".-"
	morse[1] = "-..."
	morse[2] = "-.-."
	morse[3] = "-.."
	morse[4] = "."
	morse[5] = "..-."
	morse[6] = "--."
	morse[7] = "...."
	morse[8] = ".."
	morse[9] = ".---"
	morse[10] = "-.-"
	morse[11] = ".-.."
	morse[12] = "--"
	morse[13] = "-."

	/// uggggghhhh
	morse[14] = ".-"

	morse[15] = "---"
	morse[16] = ".--."
	morse[17] = "--.-"
	morse[18] = ".-."
	morse[19] = "..."
	morse[20] = "-"
	morse[21] = "..-"
	morse[22] = "...-"
	morse[23] = ".--"
	morse[24] = "-..-"
	morse[25] = "-.--"
	morse[26] = "--.."
	morse[27] = "..---"
	morse[28] = "...--"
	morse[29] = "....-"
	morse[30] = "....."
	morse[31] = "-...."
	morse[32] = "--..."
	morse[33] = "---.."
	morse[34] = "----."
	morse[35] = "-----"
	morse[36] = "|"



	var test = "hello this is a test string, how's it going david? you enjoying this program?$"

	println(test)

	var index = 0
	for {
		var c = string(test[index])

		if c == "$" {
			break
		}

		toMorse(c, morse)
	}
}

func toMorse(x string, morse [37]string) {
	switch x {
	case "a":
		print(morse[0])
	case "b":
		print(morse[1])
	case "c":
		print(morse[2])
	case "d":
		print(morse[3])
	case "e":
		print(morse[4])
	case "f":
		print(morse[5])
	case "g":
		print(morse[6])
	case "h":
		print(morse[7])
	case "i":
		print(morse[8])
	case "j":
		print(morse[9])
	case "k":
		print(morse[10])
	case "l":
		print(morse[11])
	case "m":
		print(morse[12])
	case "n":
		print(morse[13])
	case "o":
		print(morse[15])
	case "p":
		print(morse[16])
	case "q":
		print(morse[17])
	case "r":
		print(morse[18])
	case "s":
		print(morse[19])
	case "t":
		print(morse[20])
	case "u":
		print(morse[21])
	case "v":
		print(morse[22])
	case "w":
		print(morse[23])
	case "x":
		print(morse[24])
	case "y":
		print(morse[25])
	case "z":
		print(morse[26])
	case "1":
		print(morse[27])
	case "2":
		print(morse[28])
	case "3":
		print(morse[29])
	case "4":
		print(morse[30])
	case "5":
		print(morse[31])
	case "6":
		print(morse[32])
	case "7":
		print(morse[33])
	case "8":
		print(morse[34])
	case "9":
		print(morse[35])
	case " ":
		print(morse[36])

	}
}

