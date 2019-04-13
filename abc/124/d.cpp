#include <bits/stdc++.h>

using namespace std;

typedef long long ll;
typedef vector<int> vi;
typedef vector<ll> vl;
typedef pair<int, int> pii;
typedef pair<ll, ll> pll;

typedef vector<int> va;
typedef vector<int> vb;
typedef vector<int> vc;

typedef int _loop_int;
#define REP(i, n) for (_loop_int i = 0; i < (_loop_int)(n); ++i)
#define FOR(i, a, b) for (_loop_int i = (_loop_int)(a); i < (_loop_int)(b); ++i)
#define FORR(i, a, b)                                                          \
  for (_loop_int i = (_loop_int)(b)-1; i >= (_loop_int)(a); --i)

#define DEBUG(x) cout << #x << ": " << x << endl
#define DEBUG_ARRY(v, n) REP(i, n) cout << " " << v[i]
#define DEBUG_VEC(v)                                                           \
  cout << #v << ":";                                                           \
  REP(i, v.size()) cout << " " << v[i];                                        \
  cout << endl
#define ALL(a) (a).begin(), (a).end()

#define CHMIN(a, b) a = min((a), (b))
#define CHMAX(a, b) a = max((a), (b))

// mod
const ll MOD = 1000000007ll;
#define FIX(a) ((a) % MOD + MOD) % MOD

// floating
typedef double Real;
const Real EPS = 1e-11;
#define EQ0(x) (abs(x) < EPS)
#define EQ(a, b) (abs(a - b) < EPS)
typedef complex<Real> P;

int search(char zo, string s, int max[2]) {
  int max_start = 0;
  int max_end = 0;
  int cur_start = -1;
  int cur_end = -1;
  REP(i, s.length()) {
    if (s[i] == zo) {
      if (cur_start != -1) {
        cur_end = i;
        if (max_end - max_start <= cur_end - cur_start) {
          max_start = cur_start;
          max_end = cur_end - 1;
        }
      }
      cur_start = -1;
      cur_end = -1;
      cout << s[i] << ' ' << max_start << ' ' << max_end << ' ' << cur_start
           << ' ' << cur_end << endl;

    } else {
      if (cur_start == -1 && cur_end == -1) {
        cur_start = i;
        cur_end = i;
      }
      cout << s[i] << ' ' << max_start << ' ' << max_end << ' ' << cur_start
           << ' ' << cur_end << endl;
    }
  }
  cout << max_start << ' ' << max_end << ' ' << cur_start << ' ' << cur_end
       << endl;
  return max_end - max_start + 1;
}

int main() {
  int n, k;
  string s;
  cin >> n >> k >> s;
  cout << n << ' ' << k << ' ' << s << endl;
  int res = search('1', s);
  cout << "res: " << res << endl;
  int zero = search('0', s);
  cout << "zero: " << zero << endl;

  if (res >= n) {
    cout << res << endl;
    return 0;
  }
  return 0;
}
