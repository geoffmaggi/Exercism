using System;

public static class Raindrops {
	public static string Convert(int number) {
		string ret = string.Empty;
		if(number % 3 == 0) {
			ret += "Pling";
		}
		if (number % 5 == 0) {
			ret += "Plang";
		}
		if (number % 7 == 0) {
			ret += "Plong";
		}
		return ret == string.Empty ? number.ToString() : ret;
	}
}