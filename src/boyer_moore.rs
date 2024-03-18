/// Implements the Booyer Moore voting algorithm.
///
/// This algorithm finds if any candidate has votes exceeding 50%.
/// It has a linear O(n) time complexity and constant O(1) space complexity.
///
/// We keep a running "candidate" and iterate through the votes. If the vote matches the candidate, we
/// add to its running vote tally. If not, we decrement the candidate's running vote tally. If the
/// candidate's votes drop to 0, replace it with the new candidate vote.
///
/// At the end, we perform one final count of our final candidate to ensure that candidate has exceeded
/// the greater than 50% vote threshold.
pub fn boyer_moore(votes: Vec<&str>) -> Option<&str> {
    // No votes means no candidate to win the vote
    if votes.len() == 0 {
        return None;
    }

    // The current leading candidate. We start with the first vote.
    let mut candidate = *votes.get(0).expect("Votes should not be empty");
    // Running tally of votes for `candidate`.
    let mut vote_count = 1;

    for vote in votes.iter().skip(1) {
        if *vote == candidate {
            // Our leading `candidate` gets another vote.
            vote_count += 1;
        } else if vote_count == 0 {
            // The current `candidate` has 0 net votes, this vote becomes the new `candidate`.
            candidate = vote;
        } else {
            // This vote is not for the `candidate`, decrement their running tally.
            vote_count -= 1;
        }
    }

    // For our `candidate` to win, they must get greater than 50% of the votes.
    let threshold = ((votes.len() as f64) / (2 as f64)).floor() as usize;
    let end_candidate_count = votes.iter().filter(|&vote| *vote == candidate).count();

    if end_candidate_count > threshold {
        Some(candidate)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_votes_has_no_winner() {
        assert!(boyer_moore(vec![]).is_none());
    }

    #[test]
    fn one_vote_is_the_winner() {
        let vote = "a";
        assert!(boyer_moore(vec![vote]).expect("There should be a winner") == vote);
    }

    #[test]
    fn tied_votes_has_no_winner() {
        assert!(boyer_moore(vec!["a", "b"]).is_none());
    }

    #[test]
    fn winner_first_vote() {
        let winner = "a";
        let votes = vec![winner, winner, "b"];

        assert!(boyer_moore(votes).expect("There should be a winner") == winner)
    }

    #[test]
    fn winner_last_vote() {
        let winner = "a";
        let votes = vec!["b", winner, winner];

        assert!(boyer_moore(votes).expect("There should be a winner") == winner)
    }

    #[test]
    fn many_votes() {
        let winner = "a";
        let votes = vec!["b", winner, winner, "b", winner, winner, "b"];

        assert!(boyer_moore(votes).expect("There should be a winner") == winner)
    }

    #[test]
    fn many_votes_no_greater_than_50_pct_winner() {
        let winner = "a";
        let votes = vec!["b", winner, "b", winner, winner, "c", "c"];

        assert!(boyer_moore(votes).is_none());
    }
}
