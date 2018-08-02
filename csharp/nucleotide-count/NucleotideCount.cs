using System;
using System.Collections.Generic;

public class NucleotideCount {
	public Dictionary<char, int> nucleotideCounts = new Dictionary<char, int> {
		{'A', 0},
		{'T', 0},
		{'C', 0},
		{'G', 0}
	};

	public NucleotideCount(string sequence) {
		foreach (char nucleotide in sequence) {
			try {
				this.NucleotideCounts[nucleotide]++;
			}
			catch {
				throw new InvalidNucleotideException();
			}
		}
	}

	public IDictionary<char, int> NucleotideCounts {
		get {
			return this.nucleotideCounts;
		}
	}
}

public class InvalidNucleotideException : Exception { }
