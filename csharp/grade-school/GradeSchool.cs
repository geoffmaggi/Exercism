using System;
using System.Collections.Generic;
using System.Linq;

public class School {
	public Dictionary<int, SortedSet<string>> roster = new Dictionary<int, SortedSet<string>>();

	public void Add(string student, int grade) {
		if (!this.roster.ContainsKey(grade)) {
			roster[grade] = new SortedSet<string>();
		}
		roster[grade].Add(student);
	}

	public IEnumerable<string> Roster() {
		return roster.OrderBy(x => x.Key).SelectMany(x => x.Value);
	}

	public IEnumerable<string> Grade(int grade) {
		return roster.ContainsKey(grade) ? roster[grade] : new SortedSet<string>();
	}
}