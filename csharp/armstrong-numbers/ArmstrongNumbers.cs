using System;
using System.Linq;

public static class ArmstrongNumbers {
	public static bool IsArmstrongNumber(int number) {
		var digits = number.ToString().ToCharArray();
		var length = digits.Length;
		return number == digits.Select(x => Math.Pow(Char.GetNumericValue(x), length)).Sum();
	}
}